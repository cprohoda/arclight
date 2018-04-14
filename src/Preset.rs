trait Preset {
    type PresetType;

    fn default(&self) -> DefaulPresetType;
    fn current(&self) -> Self::PresetType;
    fn set(&self, Self::PresetType) -> Result<(),PresetErr>;
}

enum DefaulPresetType {
    default,
    bigdata,
    smalldata,
    webfront,
    webback,
    wasm,
}

enum PresetErr {
	failed,
}