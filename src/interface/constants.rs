use crate::enum_as_store;

enum_as_store! {
    EffectMap: String {
        Reset => "\x1b[39;49;00m".to_string(),
        Clear => "\x1b[2J".to_string(),

        Bold => "\x1b[39;01m".to_string(),
        Underline => "\x1b[39;04m".to_string(),

        Cyan => "\x1b[01;36m".to_string(),
        Green => "\x1b[01;32m".to_string()
    },

    ResponseMap: String {
        Control => format!(
            r"{}{}----- ----- ----- ----- ----- -----
     ▄▄·  ▄ .▄▄▄▄ ..▄▄ · .▄▄ · 
    ▐█ ▌▪██▪▐█▀▄.▀·▐█ ▀. ▐█ ▀. 
    ██ ▄▄██▀▀█▐▀▀▪▄▄▀▀▀█▄▄▀▀▀█▄
    ▐███▌██▌▐▀▐█▄▄▌▐█▄▪▐█▐█▄▪▐█
    ·▀▀▀ ▀▀▀ · ▀▀▀  ▀▀▀▀  ▀▀▀▀
----- ----- ----- ----- ----- -----{}

--- --- --- --- --- ---
» {}|  Control Panel  |{}
--- --- --- --- --- ---

 • Press {}[1]{} to start a new game.
 • Press {}[2]{} to start a saved game.
 • Press {}[0]{} to exit.

[ Response ] ",
            EffectMap::Clear.value(),
            EffectMap::Bold.value(),
            EffectMap::Reset.value(),
            EffectMap::Bold.value(),
            EffectMap::Reset.value(),
            EffectMap::Cyan.value(),
            EffectMap::Reset.value(),
            EffectMap::Green.value(),
            EffectMap::Reset.value(),
            EffectMap::Bold.value(),
            EffectMap::Reset.value()
        ),

        PlrDataWhite => format!(
            "{}[ Q ] Please enter the playing mode for {}white{}:",
            EffectMap::Clear.value(),
            EffectMap::Underline.value(),
            EffectMap::Reset.value()
        ),

        PlrDataBlack => format!(
            "{}[ Q ] Please enter the playing mode for {}black{}:",
            EffectMap::Clear.value(),
            EffectMap::Underline.value(),
            EffectMap::Reset.value()
        ),

        PlrMoveSelection => format!(
            "     • Press [i] to use {}interactive{} mode.
      • Press [f] to use {}from_square-to_square{} mode.
      • Press [l] for {}legal notation{} mode.
[ Response ] ",
            EffectMap::Underline.value(),
            EffectMap::Reset.value(),
            EffectMap::Underline.value(),
            EffectMap::Reset.value(),
            EffectMap::Underline.value(),
            EffectMap::Reset.value()
        )
    }
}
