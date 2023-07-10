/*
 * File: SetupTwinCATcs
 * Project: TwinCATAUTDServer
 * Created Date: 05/08/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

using System;
using System.Collections.Generic;
using System.Xml;
using TCatSysManagerLib;
using TwinCAT.SystemManager;
using EnvDTE;
using EnvDTE80;
using System.IO;
using System.Linq;
using System.Net;
using System.Runtime.InteropServices;
using System.Runtime.InteropServices.ComTypes;
using System.Xml.Linq;
using TwinCAT.Ads;

namespace TwinCATAUTDServer
{
    internal class SetupTwinCAT
    {
        private const string SolutionName = "TwinCATAUTDServer";
        private const int HeadSize = 64;
        private const int BodySize = 249;

        private readonly string _clientIpAddr;
        private readonly SyncMode _syncMode;
        private readonly int _taskCycleTime;
        private readonly int _cpuBaseTime;
        private readonly int _sync0CycleTime;
        private readonly bool _keep;

        internal SetupTwinCAT(string clientIpAddr, SyncMode syncMode, int taskCycleTime, int cpuBaseTime, int sync0CycleTime, bool keep)
        {
            _clientIpAddr = clientIpAddr;
            _syncMode = syncMode;
            _taskCycleTime = taskCycleTime;
            _cpuBaseTime = cpuBaseTime;
            _sync0CycleTime = sync0CycleTime;
            _keep = keep;
        }

        [STAThread]
        public void Run()
        {
            var solutionPath = Path.Combine(Environment.GetEnvironmentVariable("temp") ?? string.Empty, SolutionName);
            MessageFilter.Register();
            try
            {
                // Close all TwinCAT Autd Server solutions currently opened
                var processes = System.Diagnostics.Process.GetProcesses().Where(x => x.MainWindowTitle.StartsWith(SolutionName) && x.ProcessName.Contains("devenv"));
                foreach (var process in processes) GetDte(process.Id)?.Quit();

                IPAddress.TryParse(_clientIpAddr ?? string.Empty, out var ipAddr);

                Console.WriteLine("Connecting to TcXaeShell DTE...");
                var t = Type.GetTypeFromProgID("TcXaeShell.DTE.15.0");
                var dte = (DTE2)Activator.CreateInstance(t);

                dte.SuppressUI = false;
                dte.MainWindow.Visible = true;
                dte.UserControl = true;

                Console.WriteLine("Switching TwinCAT3 to Config Mode...");
                SetConfigMode();
                System.Threading.Thread.Sleep(1000);
                Console.WriteLine("Creating a Project...");
                var project = CreateProject(dte, solutionPath);
                ITcSysManager sysManager = project.Object;
                if (ipAddr != null)
                {
                    Console.WriteLine("Setting up the Routing Table to " + ipAddr);
                    AddRoute(sysManager, ipAddr);
                }
                Console.WriteLine("Scanning Devices...");
                var autds = ScanAutDs(sysManager);
                AssignCpuCores(sysManager);
                SetupTask(sysManager, autds);
                Console.WriteLine("Activating and Restarting TwinCAT3...");
                sysManager.ActivateConfiguration();
                sysManager.StartRestartTwinCAT();
                Console.WriteLine($"Saving the Project...");
                SaveProject(dte, project, solutionPath);
                if (!_keep) dte.Quit();
            }
            catch (Exception e)
            {
                Console.Write("Error: ");
                Console.WriteLine(e.Message);
            }

            MessageFilter.Revoke();
        }

        [DllImport("ole32.dll")]
        private static extern int CreateBindCtx(uint reserved, out IBindCtx ppbc);

        public static DTE GetDte(int processId)
        {
            var progId = "!TcXaeShell.DTE.15.0:" + processId;
            object runningObject = null;

            IBindCtx bindCtx = null;
            IRunningObjectTable rot = null;
            IEnumMoniker enumMonikers = null;

            try
            {
                Marshal.ThrowExceptionForHR(CreateBindCtx(0, out bindCtx));
                bindCtx.GetRunningObjectTable(out rot);
                rot.EnumRunning(out enumMonikers);

                var moniker = new IMoniker[1];
                var numberFetched = IntPtr.Zero;
                while (enumMonikers.Next(1, moniker, numberFetched) == 0)
                {
                    var runningObjectMoniker = moniker[0];
                    string name = null;
                    try
                    {
                        runningObjectMoniker?.GetDisplayName(bindCtx, null, out name);
                    }
                    catch (UnauthorizedAccessException)
                    {
                        // Do nothing, there is something in the ROT that we do not have access to.
                    }

                    if (string.IsNullOrEmpty(name) || !string.Equals(name, progId, StringComparison.Ordinal)) continue;
                    Marshal.ThrowExceptionForHR(rot.GetObject(runningObjectMoniker, out runningObject));
                    break;
                }
            }
            finally
            {
                if (enumMonikers != null) Marshal.ReleaseComObject(enumMonikers);
                if (rot != null) Marshal.ReleaseComObject(rot);
                if (bindCtx != null) Marshal.ReleaseComObject(bindCtx);
            }
            return (DTE)runningObject;
        }

        private static void SetConfigMode()
        {
            var client = new TcAdsClient();
            var mode = new StateInfo();

            client.Connect((int)AmsPort.SystemService);
            mode.AdsState = client.ReadState().AdsState;
            mode.AdsState = AdsState.Reconfig;
            client.WriteControl(mode);
            client.Dispose();
        }

        private static void DeleteDirectory(string path)
        {
            foreach (var directory in Directory.GetDirectories(path))
                DeleteDirectory(directory);

            try
            {
                Directory.Delete(path, true);
            }
            catch (IOException)
            {
                Directory.Delete(path, true);
            }
            catch (UnauthorizedAccessException)
            {
                Directory.Delete(path, true);
            }
        }

        private static Project CreateProject(DTE2 dte, string path)
        {
            if (Directory.Exists(path))
                DeleteDirectory(path);
            Directory.CreateDirectory(path);

            var solution = dte.Solution as Solution2;
            solution.Create(path, SolutionName);
            solution.SaveAs(Path.Combine(path, SolutionName + ".sln"));

            const string template = @"C:\TwinCAT\3.1\Components\Base\PrjTemplate\TwinCAT Project.tsproj"; //path to project template
            return solution.AddFromTemplate(template, path, SolutionName);
        }

        private static void SaveProject(DTE2 dte, Project project, string path)
        {
            project.Save();
            dte.Solution.SaveAs(Path.Combine(path, SolutionName + ".sln"));
            Console.WriteLine("The Solution was saved at " + path + ".");
        }

        private static void AddRoute(ITcSysManager sysManager, IPAddress ipAddr)
        {
            var routeConfiguration = sysManager.LookupTreeItem("TIRR");
            var addProjectRouteIp = @"<TreeItem>
                                           <RoutePrj>
                                             <AddProjectRoute>
                                               <Name>" + ipAddr + @"</Name>
                                               <NetId>" + ipAddr + @".1.1" + @"</NetId>
                                               <IpAddr>" + ipAddr + @"</IpAddr>
                                             </AddProjectRoute>
                                           </RoutePrj>
                                         </TreeItem>";

            routeConfiguration.ConsumeXml(addProjectRouteIp);
        }

        private List<ITcSmTreeItem> ScanAutDs(ITcSysManager sysManager)
        {
            var devices = (ITcSmTreeItem3)sysManager.LookupTreeItem("TIID");
            var doc = new XmlDocument();
            var xml = devices.ProduceXml(false);
            doc.LoadXml(xml);
            var nodes = doc.SelectNodes("TreeItem/DeviceGrpDef/FoundDevices/Device");
            var ethernetNodes = (from XmlNode node in nodes let typeNode = node.SelectSingleNode("ItemSubType") let subType = int.Parse(typeNode.InnerText) where subType == (int)DeviceType.EtherCAT_AutomationProtocol || subType == (int)DeviceType.EtherCAT_DirectMode || subType == (int)DeviceType.EtherCAT_DirectModeV210 select node).ToList();

            if (ethernetNodes.Count == 0)
                throw new Exception("No devices were found. Check if TwinCAT3 is in Config Mode");

            Console.WriteLine("Scan found a RT-compatible Ethernet device.");
            var device = (ITcSmTreeItem3)devices.CreateChild("EtherCAT Master", (int)DeviceType.EtherCAT_DirectMode, null);

            // Taking only the first found Ethernet Adapter
            var ethernetNode = ethernetNodes[0];
            var addressInfoNode = ethernetNode.SelectSingleNode("AddressInfo");
            addressInfoNode.SelectSingleNode("Pnp/DeviceDesc").InnerText = "TwincatEthernetDevice";
            // Set the Address Info
            var xml2 = $"<TreeItem><DeviceDef>{addressInfoNode.OuterXml}</DeviceDef></TreeItem>";
            device.ConsumeXml(xml2);

            const string scanXml = "<TreeItem><DeviceDef><ScanBoxes>1</ScanBoxes></DeviceDef></TreeItem>";
            device.ConsumeXml(scanXml);
            var autds = new List<ITcSmTreeItem>();
            foreach (ITcSmTreeItem box in device)
            {
                if (box.ItemSubTypeName != "AUTD") continue;
                var bdoc = new XmlDocument();
                var bxml = box.ProduceXml(false);
                bdoc.LoadXml(bxml);

                // set DC
                if (_syncMode == SyncMode.DC)
                {
                    var dcOpmodes = bdoc.SelectNodes("TreeItem/EtherCAT/Slave/DC/OpMode");
                    foreach (XmlNode item in dcOpmodes)
                    {
                        if (item.SelectSingleNode("Name")?.InnerText == "DC")
                        {
                            var attr = bdoc.CreateAttribute("Selected");
                            attr.Value = "true";
                            item.Attributes?.SetNamedItem(attr);

                            item.SelectSingleNode("CycleTimeSync0").InnerText = _sync0CycleTime.ToString();
                            attr = bdoc.CreateAttribute("Factor");
                            attr.Value = "0";
                            item.Attributes?.SetNamedItem(attr);
                            item.SelectSingleNode("CycleTimeSync0").Attributes?.SetNamedItem(attr);
                        }
                        else
                        {
                            item.Attributes?.RemoveAll();
                        }
                    }
                }
                else
                {
                    var dcOpmodes = bdoc.SelectNodes("TreeItem/EtherCAT/Slave/DC/OpMode");
                    foreach (XmlNode item in dcOpmodes)
                    {
                        if (item.SelectSingleNode("Name")?.InnerText == "Synchron")
                        {
                            var attr = bdoc.CreateAttribute("Selected");
                            attr.Value = "true";
                            item.Attributes?.SetNamedItem(attr);

                            item.SelectSingleNode("AssignActivate").InnerText = "#x300";

                            item.SelectSingleNode("CycleTimeSync0").InnerText = _sync0CycleTime.ToString();
                            attr = bdoc.CreateAttribute("Factor");
                            attr.Value = "0";
                            item.Attributes?.SetNamedItem(attr);
                            item.SelectSingleNode("CycleTimeSync0").Attributes?.SetNamedItem(attr);
                        }
                        else
                        {
                            item.Attributes?.RemoveAll();
                        }
                    }
                }

                box.ConsumeXml(bdoc.OuterXml);

                autds.Add(box);
            }

            Console.WriteLine($"{autds.Count} AUTDs are found and added.");

            return autds;
        }

        private void SetupTask(ITcSysManager sysManager, IReadOnlyCollection<ITcSmTreeItem> autds)
        {
            var tasks = sysManager.LookupTreeItem("TIRT");
            var task1 = tasks.CreateChild("Task 1", 0, null);
            var doc = new XmlDocument();
            var xml = task1.ProduceXml(false);
            doc.LoadXml(xml);

            doc.SelectSingleNode("TreeItem/TaskDef/CycleTime").InnerText = _taskCycleTime.ToString();
            task1.ConsumeXml(doc.OuterXml);

            var task1Out = sysManager.LookupTreeItem("TIRT^Task 1^Outputs");
            // make global header
            for (var i = 0; i < HeadSize; i++)
            {
                var name = $"header[{i}]";
                task1Out.CreateChild(name, -1, null, "WORD");
            }
            // make gain body
            for (var id = 0; id < autds.Count; id++)
            {
                for (var i = 0; i < BodySize; i++)
                {
                    var name = $"gbody[{id}][{i}]";
                    task1Out.CreateChild(name, -1, null, "WORD");
                }
            }
            var task1In = sysManager.LookupTreeItem("TIRT^Task 1^Inputs");
            for (var id = 0; id < autds.Count; id++)
            {
                var name = $"input[{id}]";
                task1In.CreateChild(name, -1, null, "WORD");
            }
            // connect links
            for (var id = 0; id < autds.Count; id++)
            {
                for (var i = 0; i < HeadSize; i++)
                {
                    var source = $"TIRT^Task 1^Outputs^header[{i}]";
                    var destination = $"TIID^EtherCAT Master^Box {id + 1} (AUTD)^RxPdo1^data[{i}]";
                    sysManager.LinkVariables(source, destination);
                }
                for (var i = 0; i < BodySize; i++)
                {
                    var source = $"TIRT^Task 1^Outputs^gbody[{id}][{i}]";
                    var destination = $"TIID^EtherCAT Master^Box {id + 1} (AUTD)^RxPdo0^data[{i}]";
                    sysManager.LinkVariables(source, destination);
                }
                {
                    var source = $"TIRT^Task 1^Inputs^input[{id}]";
                    var destination = $"TIID^EtherCAT Master^Box {id + 1} (AUTD)^TxPdo^dummy";
                    sysManager.LinkVariables(source, destination);
                }
            }
        }

        [Flags]
        public enum CpuAffinity : ulong
        {
            Cpu1 = 0x0000000000000001,
            Cpu2 = 0x0000000000000002,
            Cpu3 = 0x0000000000000004,
            Cpu4 = 0x0000000000000008,
            Cpu5 = 0x0000000000000010,
            Cpu6 = 0x0000000000000020,
            Cpu7 = 0x0000000000000040,
            Cpu8 = 0x0000000000000080,
            None = 0x0000000000000000,
            MaskSingle = Cpu1,
            MaskDual = Cpu1 | Cpu2,
            MaskQuad = MaskDual | Cpu3 | Cpu4,
            MaskHexa = MaskQuad | Cpu5 | Cpu6,
            MaskOct = MaskHexa | Cpu7 | Cpu8,
            MaskAll = 0xFFFFFFFFFFFFFFFF
        }

        public void AssignCpuCores(ITcSysManager sysManager)
        {
            var realtimeSettings = sysManager.LookupTreeItem("TIRS");
            var stringWriter = new StringWriter();
            using (var writer = XmlWriter.Create(stringWriter))
            {
                writer.WriteStartElement("TreeItem");
                writer.WriteStartElement("RTimeSetDef");
                writer.WriteElementString("MaxCPUs", "1");
                writer.WriteStartElement("CPUs");
                WriteCpuProperties(writer, 0);
                writer.WriteEndElement(); // CPUs     
                writer.WriteEndElement(); // RTimeSetDef     
                writer.WriteEndElement(); // TreeItem
            }
            var xml = stringWriter.ToString();
            realtimeSettings.ConsumeXml(xml);
        }

        private void WriteCpuProperties(XmlWriter writer, int id)
        {
            writer.WriteStartElement("CPU");
            writer.WriteAttributeString("id", id.ToString());
            writer.WriteElementString("BaseTime", _cpuBaseTime.ToString());
            writer.WriteEndElement();
        }
    }

    public class MessageFilter : IOleMessageFilter
    {
        //
        // Class containing the IOleMessageFilter
        // thread error-handling functions.

        // Start the filter.
        public static void Register()
        {
            IOleMessageFilter newFilter = new MessageFilter();
            CoRegisterMessageFilter(newFilter, out _);
        }

        // Done with the filter, close it.
        public static void Revoke()
        {
            CoRegisterMessageFilter(null, out _);
        }

        //
        // IOleMessageFilter functions.
        // Handle incoming thread requests.
        int IOleMessageFilter.HandleInComingCall(int dwCallType,
          IntPtr hTaskCaller, int dwTickCount, IntPtr
          lpInterfaceInfo)
        {
            return 0;
        }

        // Thread call was rejected, so try again.
        int IOleMessageFilter.RetryRejectedCall(IntPtr
          hTaskCallee, int dwTickCount, int dwRejectType)
        {
            return dwRejectType == 2 ? 99 : -1;
        }

        int IOleMessageFilter.MessagePending(IntPtr hTaskCallee,
          int dwTickCount, int dwPendingType)
        {
            return 2;
        }

        // Implement the IOleMessageFilter interface.
        [DllImport("Ole32.dll")]
        private static extern int
          CoRegisterMessageFilter(IOleMessageFilter newFilter, out
          IOleMessageFilter oldFilter);
    }

    [ComImport, Guid("00000016-0000-0000-C000-000000000046"),
    InterfaceTypeAttribute(ComInterfaceType.InterfaceIsIUnknown)]
    internal interface IOleMessageFilter
    {
        [PreserveSig]
        int HandleInComingCall(
            int dwCallType,
            IntPtr hTaskCaller,
            int dwTickCount,
            IntPtr lpInterfaceInfo);

        [PreserveSig]
        int RetryRejectedCall(
            IntPtr hTaskCallee,
            int dwTickCount,
            int dwRejectType);

        [PreserveSig]
        int MessagePending(
            IntPtr hTaskCallee,
            int dwTickCount,
            int dwPendingType);
    }
}
