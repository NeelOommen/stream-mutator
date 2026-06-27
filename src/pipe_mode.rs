#[derive(Clone, Copy)]
pub enum PipeMode{
    DefaultMode,
    DefaultMetricMode,
    RawResponseLoggingMode,
    HeaderInjectionMode,
}