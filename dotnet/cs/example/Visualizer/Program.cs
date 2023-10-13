/*
 * File: Program.cs
 * Project: Visualizer
 * Created Date: 13/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 13/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

//#define USE_PYTHON

using AUTD3Sharp.Utils;
using AUTD3Sharp;
using AUTD3Sharp.Gain;
using AUTD3Sharp.Link;
using AUTD3Sharp.Modulation;

#if USE_PYTHON
using PlotConfig = AUTD3Sharp.Link.PyPlotConfig; 
#else
using PlotConfig = AUTD3Sharp.Link.PlotConfig;
#endif

try
{
    var autd = Controller.Builder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).OpenWith(
        Visualizer.Builder()
#if USE_PYTHON
        .WithBackend<PythonBackend>()
#endif
    );

    var center = autd.Geometry.Center + new Vector3d(0, 0, 150);

    var g = new Focus(center);
    var m = new Square(150);

    autd.Send(m, g);

    autd.Link<Visualizer>().PlotPhase(new PlotConfig
    {
        Fname = "phase.png"
    }, autd.Geometry);

    autd.Link<Visualizer>().PlotField(new PlotConfig
    {
        Fname = "x.png"
    },
        new PlotRange
        {
            XStart = center.x - 50,
            XEnd = center.x + 50,
            YStart = center.y,
            YEnd = center.y,
            ZStart = center.z,
            ZEnd = center.z,
            Resolution = 1
        },
        autd.Geometry);


    autd.Link<Visualizer>().PlotField(new PlotConfig
    {
        Fname = "xy.png"
    },
        new PlotRange
        {
            XStart = center.x - 20,
            XEnd = center.x + 20,
            YStart = center.y - 30,
            YEnd = center.y + 30,
            ZStart = center.z,
            ZEnd = center.z,
            Resolution = 1
        },
        autd.Geometry);


    autd.Link<Visualizer>().PlotField(new PlotConfig
    {
        Fname = "yz.png"
    },
        new PlotRange
        {
            XStart = center.x,
            XEnd = center.x,
            YStart = center.y - 30,
            YEnd = center.y + 30,
            ZStart = 0,
            ZEnd = center.z + 50,
            Resolution = 2
        },
        autd.Geometry);



    autd.Link<Visualizer>().PlotField(new PlotConfig
    {
        Fname = "zx.png"
    },
        new PlotRange
        {
            XStart = center.x - 30,
            XEnd = center.x + 30,
            YStart = center.y,
            YEnd = center.y,
            ZStart = 0,
            ZEnd = center.z + 50,
            Resolution = 2
        },
        autd.Geometry);


    autd.Link<Visualizer>().PlotModulation(new PlotConfig
    {
        Fname = "mod.png"
    });

    // Calculate acoustic pressure without plotting
    var points = new List<Vector3d> { center };
    var p = autd.Link<Visualizer>().CalcField(points, autd.Geometry);
    Console.WriteLine($"Acoustic pressure at ({center.x}, {center.y}, {center.z}) = ({p[0]})");

    autd.Close();

}
catch (Exception e)
{
    Console.WriteLine(e.Message);
}
