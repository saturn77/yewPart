#![recursion_limit="2048"]

use yew::prelude::*;
use js_sys::Date; 

mod capacitor;

// Define the possible message signals 
pub enum Msg {
    Temp85,
    Temp105,
    Temp125,
    Temp150,
    X5R,
    X6S,
    X7R,
    X8R,
    P1,
    P2,
    P10,
    P20,
    ClearText,
    LowCost,
    Nominal,
    HighRel,
}

#[allow(non_snake_case)]
pub struct State{
    low_cost    : bool, 
    nominal     : bool, 
    high_rel    : bool, 
    dielectric  : capacitor::Dielectric, 
    temperature : capacitor::Temperature, 
    tolerance   : capacitor::Percent,
}

#[allow(non_snake_case)]
pub struct Model {
    state    : State,
    text     : String, 
    cap_string : String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) ->  Self {
        // let mut state = State {
        //     low_cost   : false, 
        //     nominal    : false, 
        //     high_rel   : false, 
        //     dielectric : capacitor::Dielectric::new(),
        //     temperature : capacitor::Temperature::new(),
        //     tolerance   : capacitor::Percent::new(),
        // };
        Self {
            state    : State{
                low_cost   : false, 
            nominal    : false, 
            high_rel   : false, 
            dielectric : capacitor::Dielectric::new(),
            temperature : capacitor::Temperature::new(),
            tolerance   : capacitor::Percent::new(),
            }, 
            text     :  "".to_string(),
            cap_string : "100pF 0402 16V".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Temp85 => {
                self.state.temperature.t85 = !self.state.temperature.t85; 
                true 
            }
            Msg::Temp105 => {
                self.state.temperature.t105 = !self.state.temperature.t105;
                true 
            }
            Msg::Temp125 => {
                self.state.temperature.t125 = !self.state.temperature.t125;
                true 
            }
            Msg::Temp150 => {
                self.state.temperature.t150 = !self.state.temperature.t150;
                true 
            }
            Msg::X5R => {
                self.state.dielectric.x5 = !self.state.dielectric.x5; 
                true 
            }
            Msg::X6S => {
                self.state.dielectric.x6 = !self.state.dielectric.x6; 
                true 
            }
            Msg::X7R => {
                self.state.dielectric.x7 = !self.state.dielectric.x7;
                if self.state.dielectric.x7 {
                    self.text = " ".to_string();
                    self.text += "Dielectric of X7R is now allowed\r\n";
                } 
                true
            }
            Msg::X8R => {
                self.state.dielectric.x8 = !self.state.dielectric.x8;
                if self.state.dielectric.x8 == true {
                    self.text = " ".to_string();
                    self.text += "Dielectric of X8R is now allowed\r\n";
                } 
                true 
            }
            Msg::P1 => {
                self.state.tolerance.p1 = !self.state.tolerance.p1;
                if self.state.tolerance.p1 {
                    self.text = " ".to_string();
                    self.text += "\n Selected 1% Tolerance\r\n";
                } 
                true 
            }
            Msg::P2 => {
                self.state.tolerance.p2 = !self.state.tolerance.p2;
                if self.state.tolerance.p2 {
                    self.text = " ".to_string();
                    self.text += "\n Selected 2% Tolerance\r\n";
                } 
                true 
            }
            Msg::P10 => {
                self.state.tolerance.p10 = !self.state.tolerance.p10;
                if self.state.tolerance.p10 == true {
                    self.text = " ".to_string();
                    self.text += "\r\n Selected 10% Tolerance\r\n";
                } 
                true 
            }
            Msg::P20 => {
                self.state.tolerance.p20 = !self.state.tolerance.p20;
                if self.state.tolerance.p20 == true {
                    self.text = " ".to_string();
                    self.text += "\n Selected 20% Tolerance\r\n";
                } 
                true 
            }
            Msg::ClearText => {
                self.text = " ".to_string(); 
                self.state.dielectric.clear();
                self.state.temperature.clear();
                self.state.tolerance.clear();
                self.state.low_cost = false;
                self.state.nominal  = false;
                self.state.high_rel = false;
                true 
            }
            Msg::LowCost => {
                if !self.state.low_cost {
                  self.text = " ".to_string();
                  self.text = "Low Cost search selected.".to_string();  
                  self.state.dielectric.clear();
                  self.state.temperature.clear();
                  self.state.tolerance.clear();
                  self.state.temperature.t85  = true;
                  self.state.temperature.t105 = true; 
                  self.state.dielectric.x5    = true;
                  self.state.dielectric.x6    = true;
                  self.state.tolerance.p10    = true;
                  self.state.tolerance.p20    = true; 
                  self.state.nominal          = false;
                  self.state.high_rel         = false;
                  self.state.low_cost         = true; 
                } else {
                    self.state.dielectric.clear();
                    self.state.temperature.clear();
                    self.state.tolerance.clear();
                    self.text = " ".to_string();
                    self.text = "Low Cost search removed.".to_string();
                    self.state.low_cost        = false;  
                }
                true 
            }

            Msg::Nominal => {
                self.text = " ".to_string();
                self.text = "Nominal search selected.".to_string(); 
                self.state.dielectric.clear();
                self.state.temperature.clear();
                self.state.tolerance.clear();
                self.state.temperature.t105 = true;
                self.state.temperature.t125 = true;  
                self.state.dielectric.x6   = true;
                self.state.dielectric.x7   = true; 
                self.state.tolerance.p10   = true;
                self.state.tolerance.p20   = true; 
                self.state.low_cost        = false; 
                self.state.high_rel        = false; 
                self.state.nominal         = !self.state.nominal; 
                true 
            }

            Msg::HighRel => {
                self.text = " ".to_string();
                self.text = "High Reliability search selected.".to_string();  
                self.state.dielectric.clear();
                self.state.temperature.clear();
                self.state.tolerance.clear();
                self.state.temperature.t125 = true;
                self.state.temperature.t150 = true;  
                self.state.dielectric.x7   = true;
                self.state.dielectric.x8   = true; 
                self.state.tolerance.p10   = true;
                self.state.low_cost        = false;
                self.state.nominal         = false;
                self.state.high_rel        = !self.state.high_rel; 
                true 
            }
        }
    }


    #[allow(non_snake_case)]
    fn view(&self, ctx: &Context<Self>) -> Html {
        let textbox  = &self.text; 
        let cap_stringX = &self.cap_string; 

        html! {
            <div>
                <br/>
                <br/>
                <div class="page-title"> <text>{"yewPart"}</text> </div>
                <div class="page-title"> <text>{"Integrating circuit design with software for design optimization"}</text> </div>
                <br/>
                <div class="grid-title-area">
                    <img src="/assets/cap.png" class="display-image" style="width:300px;height:200px;justify-content: center;padding:10px 0px 10px 30px" />
                    <div class="page-search"> <text>{"Capacitor String:"}</text> 
                    <div class="editor2" contenteditable="true">
                        <text> {cap_stringX} </text>
                    </div>
                    <br/>
                    <div class="grid-micro">
                    <div class="grid-item"><text>{ "Low Cost Search" }</text></div>
                    <div class="grid-item"><input type="checkbox"  checked={self.state.low_cost} onclick={ctx.link().callback(|_| Msg::LowCost)} /></div>

                    <div class="grid-item"><text>{ "Nominal" }</text></div>
                    <div class="grid-item"><input type="checkbox"  checked={self.state.nominal} onclick={ctx.link().callback(|_| Msg::Nominal)} /></div>

                    <div class="grid-item"><text>{ "High Reliability" }</text></div>
                    <div class="grid-item"><input type="checkbox"  checked={self.state.high_rel} onclick={ctx.link().callback(|_| Msg::HighRel)} /></div>
                    </div>
                    
                    </div>
                    <img src="/assets/cap_zynq.png" class="display-image" style="width:300px;height:200px;justify-content: center;padding:10px 0px 10px 30px" />
                </div>
                <br/>
                <br/>
                <div class="grid-container-main">
                        <div class="grid-container">
                        
                        <div class="grid-item"><text>{ "Temp" }</text></div>
                        <div class="grid-item"><text>{ "Enable" }</text></div>
                        <div class="grid-item"><text>{ "Filter Details" }</text></div>

                        <div class="grid-item"><text>{ "85 C" }</text></div>
                        <div class="grid-item"><input type="checkbox"  checked={self.state.temperature.t85} onclick={ctx.link().callback(|_| Msg::Temp85)} /></div>
                        <div class="grid-item"><text>{ "Allow 85C parts." }</text></div>

                        <div class="grid-item"><text>{ "105 C" }</text></div>
                        <div class="grid-item"><input type="checkbox"  checked={self.state.temperature.t105} onclick={ctx.link().callback(|_| Msg::Temp105)} /></div>
                        <div class="grid-item"><text>{ "Allow 105C parts" }</text></div>

                        <div class="grid-item"><text>{ "125 C" }</text></div>
                        <div class="grid-item"><input type="checkbox"  checked={self.state.temperature.t125} onclick={ctx.link().callback(|_| Msg::Temp125)} /></div>
                        <div class="grid-item"><text>{ "Allow 125C parts" }</text></div>
                        
                        <div class="grid-item"><text>{ "150 C" }</text></div>
                        <div class="grid-item"><input type="checkbox"  checked={self.state.temperature.t150} onclick={ctx.link().callback(|_| Msg::Temp150)} /></div>
                        <div class="grid-item"><text>{ "Allow 150C parts." }</text></div>
                    </div>

                    <div class="grid-container">
                        <div class="grid-item"><text>{ "Dielectric" }</text></div>
                        <div class="grid-item"><text>{ "Enable" }</text></div>
                        <div class="grid-item"><text>{ "Filter Details" }</text></div>

                        <div class="grid-item"><text>{ "X5 Family" }</text></div>
                        <div class="grid-item"><input type="checkbox"  checked={self.state.dielectric.x5} onclick={ctx.link().callback(|_| Msg::X5R)} /></div>
                        <div class="grid-item"><text>{ "Allow X5S, X5R." }</text></div>
                        
                        <div class="grid-item"><text>{ "X6 Family" }</text></div>
                        <div class="grid-item"><input type="checkbox"  checked={self.state.dielectric.x6} onclick={ctx.link().callback(|_| Msg::X6S)} /></div>
                        <div class="grid-item"><text>{ "Allow X6S." }</text></div>

                        <div class="grid-item"><text>{ "X7 Family" }</text></div>
                        <div class="grid-item"><input type="checkbox"  checked={self.state.dielectric.x7} onclick={ctx.link().callback(|_| Msg::X7R)} /></div>
                        <div class="grid-item"><text>{ "Allow X7R, X7S, X7T." }</text></div>
        
                        <div class="grid-item"><text>{ "X8 Family" }</text></div>
                        <div class="grid-item"><input type="checkbox"  checked={self.state.dielectric.x8} onclick={ctx.link().callback(|_| Msg::X8R)} /></div>
                        <div class="grid-item"><text>{ "Allow X8R, X8S, X8T." }</text></div>
                    </div>

                    <div class="grid-container">
                        <div class="grid-item"><text>{ "Tolerance" }</text></div>
                        <div class="grid-item"><text>{ "Enable" }</text></div>
                        <div class="grid-item"><text>{ "Filter Details" }</text></div>
                        
                        <div class="grid-item"><text>{ "1% Family" }</text></div>
                        <div class="grid-item"><input type="checkbox"  checked={self.state.tolerance.p1} onclick={ctx.link().callback(|_| Msg::P1)} /></div>
                        <div class="grid-item"><text>{ "Allow 1% change(COG)." }</text></div>

                        <div class="grid-item"><text>{ "2% Family" }</text></div>
                        <div class="grid-item"><input type="checkbox"  checked={self.state.tolerance.p2}  onclick={ctx.link().callback(|_| Msg::P2)} /></div>
                        <div class="grid-item"><text>{ "Allow 2% change." }</text></div>

                        <div class="grid-item"><text>{ "10% Family" }</text></div>
                        <div class="grid-item"><input type="checkbox"  checked={self.state.tolerance.p10} onclick={ctx.link().callback(|_| Msg::P10)} /></div>
                        <div class="grid-item"><text>{ "Allow 10% change." }</text></div>

                        <div class="grid-item"><text>{ "20% Family" }</text></div>
                        <div class="grid-item"><input type="checkbox"  checked={self.state.tolerance.p20} onclick={ctx.link().callback(|_| Msg::P20)} /></div>
                        <div class="grid-item"><text>{ "Allow 20% change." }</text></div>
                        
                    </div>
                </div>
                <p class="footer">
                    { "Rendered: "}
                    { String::from(Date::new_0().to_string()) }
                </p>
            
                <div class="grid-title-area">
                    <button class="button" style="width:140px;height:40px;justify-content: right;padding:10px 0px 10px 30px" onclick={ctx.link().callback(|_| Msg::ClearText)}> { "Clear Filters" } </button> 
                    <div class="editor1" contenteditable="true"><text>{textbox}</text></div>
                    <div class="commentX"><text>{""}</text></div>
                </div>
        </div>
                
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
