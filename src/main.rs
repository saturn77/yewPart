#![recursion_limit="2048"]

use js_sys::Date;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
mod capacitor; 

// #[derive(Debug)]
// struct State {
//     allow_x5: bool,
//     allow_x6: bool,
//     allow_x7: bool,
// }
// Define the possible messages which can be sent to the component
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
}

#[allow(non_snake_case)]
pub struct State{
    dielectric  : capacitor::Dielectric, 
    temperature : capacitor::Temperature, 
    tolerance   : capacitor::Percent,
}

#[allow(non_snake_case)]
pub struct Model {
    link: ComponentLink<Self>,
    state    : State,
    text     : String, 
    cap_string : String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
            dielectric : capacitor::Dielectric::new(),
            temperature : capacitor::Temperature::new(),
            tolerance   : capacitor::Percent::new(),
        };
        Self {
            link , 
            state,
            text     :  "".to_string(),
            cap_string : "100pF 0402 16V".to_string(),
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
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
                true 
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    #[allow(non_snake_case)]
    fn view(&self) -> Html {
        let textbox  = &self.text; 
        let cap_stringX = &self.cap_string; 

        html! {
            <div>
                <br/>
                <br/>
                <div class="page-title"> <text>{"AtlantixEDA AlphaPart"}</text> </div>
                <div class="page-title"> <text>{"Providing Intelligence Based Part Search Results"}</text> </div>
                <br/>
                <div class="grid-title-area">
                    <img src="cap.png" class="display-image" style="width:300px;height:200px;justify-content: center;padding:10px 0px 10px 30px" />
                    <div class="page-search"> <text>{"Capacitor String:"}</text> 
                    <div class="editor2" contenteditable="true">
                        <text> {cap_stringX} </text>
                    </div>
                    <label>{ "Low Cost Search" }</label> 
                    <input type="checkbox"  checked=self.state.temperature.t105 onclick=self.link.callback(|_| Msg::Temp105) />
                    <br/>
                    <label>{ "Nominal Search" }</label> 
                    <input type="checkbox"  checked=self.state.temperature.t125 onclick=self.link.callback(|_| Msg::Temp125) />
                    <br/>
                    <label>{ "High Reliability" }</label> 
                    <input type="checkbox"  checked=self.state.temperature.t150 onclick=self.link.callback(|_| Msg::Temp150) />
                    
                    </div>
                    <img src="cap_zynq.png" class="display-image" style="width:300px;height:200px;justify-content: center;padding:10px 0px 10px 30px" />
                </div>
                <br/>
                <br/>
                <div class="grid-container-main">
                        <div class="grid-container">
                        
                        <div class="grid-item"><text>{ "Temp" }</text></div>
                        <div class="grid-item"><text>{ "Enable" }</text></div>
                        <div class="grid-item"><text>{ "Filter Details" }</text></div>

                        <div class="grid-item"><text>{ "85 C" }</text></div>
                        <div class="grid-item"><input type="checkbox"  checked=self.state.temperature.t85 onclick=self.link.callback(|_| Msg::Temp85) /></div>
                        <div class="grid-item"><text>{ "Allow 85C parts." }</text></div>

                        <div class="grid-item"><text>{ "105 C" }</text></div>
                        <div class="grid-item"><input type="checkbox"  checked=self.state.temperature.t105 onclick=self.link.callback(|_| Msg::Temp105) /></div>
                        <div class="grid-item"><text>{ "Allow 105C parts" }</text></div>

                        <div class="grid-item"><text>{ "125 C" }</text></div>
                        <div class="grid-item"><input type="checkbox"  checked=self.state.temperature.t125 onclick=self.link.callback(|_| Msg::Temp125) /></div>
                        <div class="grid-item"><text>{ "Allow 125C parts" }</text></div>
                        
                        <div class="grid-item"><text>{ "150 C" }</text></div>
                        <div class="grid-item"><input type="checkbox"  checked=self.state.temperature.t150 onclick=self.link.callback(|_| Msg::Temp150) /></div>
                        <div class="grid-item"><text>{ "Allow 150C parts." }</text></div>
                    </div>

                    <div class="grid-container">
                        <div class="grid-item"><text>{ "Dielectric" }</text></div>
                        <div class="grid-item"><text>{ "Enable" }</text></div>
                        <div class="grid-item"><text>{ "Filter Details" }</text></div>

                        <div class="grid-item"><text>{ "X5 Family" }</text></div>
                        <div class="grid-item"><input type="checkbox"  checked=self.state.dielectric.x5 onclick=self.link.callback(|_| Msg::X5R) /></div>
                        <div class="grid-item"><text>{ "Allow X5S, X5R." }</text></div>
                        
                        <div class="grid-item"><text>{ "X6 Family" }</text></div>
                        <div class="grid-item"><input type="checkbox"  checked=self.state.dielectric.x6 onclick=self.link.callback(|_| Msg::X6S) /></div>
                        <div class="grid-item"><text>{ "Allow X6S." }</text></div>

                        <div class="grid-item"><text>{ "X7 Family" }</text></div>
                        <div class="grid-item"><input type="checkbox"  checked=self.state.dielectric.x7 onclick=self.link.callback(|_| Msg::X7R) /></div>
                        <div class="grid-item"><text>{ "Allow X7R, X7S, X7T." }</text></div>
        
                        <div class="grid-item"><text>{ "X8 Family" }</text></div>
                        <div class="grid-item"><input type="checkbox"  checked=self.state.dielectric.x8 onclick=self.link.callback(|_| Msg::X8R) /></div>
                        <div class="grid-item"><text>{ "Allow X8R, X8S, X8T." }</text></div>
                    </div>

                    <div class="grid-container">
                        <div class="grid-item"><text>{ "Tolerance" }</text></div>
                        <div class="grid-item"><text>{ "Enable" }</text></div>
                        <div class="grid-item"><text>{ "Filter Details" }</text></div>
                        
                        <div class="grid-item"><text>{ "1% Family" }</text></div>
                        <div class="grid-item"><input type="checkbox"  checked=self.state.tolerance.p1 onclick=self.link.callback(|_| Msg::P1) /></div>
                        <div class="grid-item"><text>{ "Allow 1% change(COG)." }</text></div>

                        <div class="grid-item"><text>{ "2% Family" }</text></div>
                        <div class="grid-item"><input type="checkbox"  checked=self.state.tolerance.p2  onclick=self.link.callback(|_| Msg::P2) /></div>
                        <div class="grid-item"><text>{ "Allow 2% change." }</text></div>

                        <div class="grid-item"><text>{ "10% Family" }</text></div>
                        <div class="grid-item"><input type="checkbox"  checked=self.state.tolerance.p10 onclick=self.link.callback(|_| Msg::P10) /></div>
                        <div class="grid-item"><text>{ "Allow 10% change." }</text></div>

                        <div class="grid-item"><text>{ "20% Family" }</text></div>
                        <div class="grid-item"><input type="checkbox"  checked=self.state.tolerance.p20 onclick=self.link.callback(|_| Msg::P20) /></div>
                        <div class="grid-item"><text>{ "Allow 20% change." }</text></div>
                        
                    </div>
                </div>
                <p class="footer">
                    { "Rendered: "}
                    { String::from(Date::new_0().to_string()) }
                </p>
            
                <div class="grid-title-area">
                    <button class="button" style="width:140px;height:40px;justify-content: right;padding:10px 0px 10px 30px" onclick=self.link.callback(|_| Msg::ClearText)> { "Clear Filters" } </button> 
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
