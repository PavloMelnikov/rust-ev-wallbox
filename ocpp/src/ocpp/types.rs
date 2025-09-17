



enum Location {
    Body,
    Cable,
    ChargingStation,
    Connector,
    Evse,
    EvseBox,
    Gps,
    Other,
    PowerOutlet,
    PowerSupply,
    Room,
}

enum MessageTrigger {
    BootNotification,
    DiagnosticsStatusNotification,
    FirmwareStatusNotification,
    Heartbeat,
    MeterValues,
    StatusNotification
}

enum Measurand {
    CurrentExport,
    CurrentImport,
    CurrentOffered,
    CurrentAvailable,
    EnergyActiveExportRegister,
    EnergyActiveImportRegister,
    EnergyReactiveExportRegister,
    EnergyReactiveImportRegister,
    EnergyActiveExportInterval,
    EnergyActiveImportInterval,
    EnergyReactiveExportInterval,
    Voltage,
    PowerActiveExport,
    PowerActiveImport,
    PowerFactor,
    PowerOffered,
    PowerReactiveExport,
    PowerReactiveImport,
    PowerApparent,
    Frequency,
    Resistance,
    CommonTemperature,
    RPM,
    SoC
}

enum ReadingContext {
    InterruptionBegin,
    InterruptionEnd,
    SampleClock,
    SamplePeriodic,
    TransactionBegin,
    TransactionEnd,
    Trigger,
    Other,
}

enum  Reason {
    EmergencyStop,
    EVDisconnected,
    HardReset,
    Local,
    Other,
    PowerLoss,
    PowerSwitchOff,
    PowerSwitchOn,
    Remote,
    SoftReset,
    Unknown,
    WakeUp,
    UnlockCommand
}

enum RecurrencyKind {
    Daily,
    Hourly,
    Minutely,
    Monthly,
    Weekly,
    Yearly
}

enum RegistrationStatus {
    Accepted,
    Blocked,
    Deleted,
    Pending,
    Rejected
}

enum RemoteStartStopStatus {
    Accepted,
    Rejected
}

enum ReservationStatus {
    Accepted,
    Faulted,
    Occupied,
    Rejected,
    Unavailable
}

enum ResetStatus {
    Accepted,
    Rejected
}

enum ResetType {
    Hard,
    Soft
}

pub struct SampledValue {
    value: String,
    context: ReadingContext,
    format: String,
    measurand: Measurand,
    unit: String,
}
pub struct MeterValue {
    timestamp: String,
    value: String,

}

