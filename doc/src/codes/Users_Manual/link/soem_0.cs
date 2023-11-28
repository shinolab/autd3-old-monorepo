using AUTD3Sharp;
using AUTD3Sharp.Link;

var onLost = new SOEM.OnErrCallbackDelegate(msg =>
{
    Console.WriteLine($"Unrecoverable error occurred: {msg}");
    Environment.Exit(-1);
});
var onErr = new SOEM.OnErrCallbackDelegate(msg =>
{
    Console.WriteLine($"Err: {msg}");
});

SOEM.Builder()
    .WithIfname("")
    .WithBufSize(32)
    .WithOnErr(onErr)
    .WithStateCheckInterval(TimeSpan.FromMilliseconds(100))
    .WithOnLost(onLost)
    .WithSync0Cycle(2)
    .WithSendCycle(2)
    .WithTimerStrategy(TimerStrategy.BusyWait)
    .WithSyncMode(SyncMode.DC)