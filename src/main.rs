#![recursion_limit="2048"]

use js_sys::Date;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
mod capacitor; 
use capacitor::{Dielectric};
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
}

#[allow(non_snake_case)]
pub struct State{
    dielectric : capacitor::Dielectric, 
    flag_85  : bool,
    flag_105 : bool,
    flag_125 : bool,
    flag_150 : bool, 
    flag_P1  : bool,
    flag_P2  : bool,  
    flag_P10 : bool,
    flag_P20 : bool,
}

#[allow(non_snake_case)]
pub struct Model {
    link: ComponentLink<Self>,
    state    : State,
    // flag_125 : bool,
    // flag_150 : bool, 
    // flag_X5R : bool,
    // flag_X6S : bool,
    // flag_X7R : bool,
    // flag_X8R : bool,
    text     : String, 
    cap_string : String,
    //state    : State,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {

        let state = State {
            dielectric : capacitor::Dielectric::new(),
            flag_85  : false,
            flag_105 : false,
            flag_125: false,
            flag_150 : false,
            flag_P1  : false, 
            flag_P2  : false, 
            flag_P10 : false,
            flag_P20 : false,
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
                self.state.flag_85 = !self.state.flag_85;
                true 
            }

            Msg::Temp105 => {
                self.state.flag_105 = !self.state.flag_105;
                true 
            }

            Msg::Temp125 => {
                self.state.flag_125 = !self.state.flag_125;
                true 
            }
            Msg::Temp150 => {
                self.state.flag_150 = !self.state.flag_150;
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
                self.state.flag_P1 = !self.state.flag_P1;
                if self.state.flag_P1 {
                    self.text = " ".to_string();
                    self.text += "\n Selected 1% Tolerance\r\n";
                } 
                true 
            }

            Msg::P2 => {
                self.state.flag_P2 = !self.state.flag_P2;
                if self.state.flag_P2 {
                    self.text = " ".to_string();
                    self.text += "\n Selected 2% Tolerance\r\n";
                } 
                true 
            }

            Msg::P10 => {
                self.state.flag_P10 = !self.state.flag_P10;
                if self.state.flag_P10 == true {
                    self.text = " ".to_string();
                    self.text += "\r\n Selected 10% Tolerance\r\n";
                } 
                true 
            }

            Msg::P20 => {
                self.state.flag_P20 = !self.state.flag_P20;
                if self.state.flag_P20 == true {
                    self.text = " ".to_string();
                    self.text += "\n Selected 20% Tolerance\r\n";
                } 
                true 
            }

        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    #[allow(non_snake_case)]
    fn view(&self) -> Html {

        // let flag_125 = self.flag_125;
        // let flag_150 = self.flag_150;
        // let flag_X5R = self.flag_X5R; 
        // let flag_X6S = self.flag_X6S;
        // let flag_X7R = self.flag_X7R;
        // let flag_X8R = self.flag_X8R;
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
                        <div class="grid-item"><input type="checkbox"  checked=self.state.flag_85 onclick=self.link.callback(|_| Msg::Temp85) /></div>
                        <div class="grid-item"><text>{ "Allow 85C parts." }</text></div>

                        <div class="grid-item"><text>{ "105 C" }</text></div>
                        <div class="grid-item"><input type="checkbox"  checked=self.state.flag_105 onclick=self.link.callback(|_| Msg::Temp105) /></div>
                        <div class="grid-item"><text>{ "Allow 105C parts" }</text></div>

                        <div class="grid-item"><text>{ "125 C" }</text></div>
                        <div class="grid-item"><input type="checkbox"  checked=self.state.flag_125 onclick=self.link.callback(|_| Msg::Temp125) /></div>
                        <div class="grid-item"><text>{ "Allow 125C parts" }</text></div>
                        
                        <div class="grid-item"><text>{ "150 C" }</text></div>
                        <div class="grid-item"><input type="checkbox"  checked=self.state.flag_150 onclick=self.link.callback(|_| Msg::Temp150) /></div>
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
                        <div class="grid-item"><input type="checkbox"  checked=self.state.flag_P1 onclick=self.link.callback(|_| Msg::P1) /></div>
                        <div class="grid-item"><text>{ "Allow 1% change(COG)." }</text></div>

                        <div class="grid-item"><text>{ "10% Family" }</text></div>
                        <div class="grid-item"><input type="checkbox"  checked=self.state.flag_P2 onclick=self.link.callback(|_| Msg::P2) /></div>
                        <div class="grid-item"><text>{ "Allow 2% change." }</text></div>

                        <div class="grid-item"><text>{ "10% Family" }</text></div>
                        <div class="grid-item"><input type="checkbox"  checked=self.state.flag_P10 onclick=self.link.callback(|_| Msg::P10) /></div>
                        <div class="grid-item"><text>{ "Allow 10% change." }</text></div>

                        <div class="grid-item"><text>{ "20% Family" }</text></div>
                        <div class="grid-item"><input type="checkbox"  checked=self.state.flag_P20 onclick=self.link.callback(|_| Msg::P20) /></div>
                        <div class="grid-item"><text>{ "Allow 20% change." }</text></div>
                        
                    </div>
                </div>
                // Display the current date and time the page was rendered
                <p class="footer">
                    { "Rendered: "}
                    { String::from(Date::new_0().to_string()) }
                </p>
            
                <div class="editor1" contenteditable="true">
                <text>    {textbox} </text>
                </div>
            </div>
                
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
