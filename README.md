# yewPart
Automating electrical engineering hardware design, providing intelligence to the process of Search and Design.

The process of selecting components when designing circuits and printed circuit boards can be quite involved. Typically capacitors offer a wide range of choices. Capacitors can also be problematic from a reliability point of view. Alpha Part addresses these issues by providing a mechanicsm for:

-- Generic search, for example, 100 pF 0402 16V
-- Async requests to Octopart API
-- Intelligent filters for the JSON responses from the Generic search
## Yew and WASM
Traditionally I have developed GUI's with Qt and Python or C++. While Qt has a long legacy and is quite powerful, the WASM environment is compelling. For example, a WASM file is easy to distribute to a large audience of users via a simple web server (say Actix or Rocket, or something like Flask) and thus provides wide user access. Alternatively, the WASM program can be run locally on localhost.

The Yew framework is inspired by Elm and React and overall I find that it has a standard type of Traits that operate on a struct - these being create ,update, change, and view. The update and change methods somewhat overall in functionality, but if you want to design a larger application with compponents and children using the change method will allow upstream components to access  your application. For smaller applications, using the update method is fine. 

The Yew enumerated Msg:: framework is analogous to Qt's signals where an event occurs and triggers a signal, or in the case of Yew, a Message. Yew specifies the use of enumerated types (Rust algebraic data types) to handle these messages.

## Search Mechanism and Program Use
Using alpha part is relatively simple. First, enter a generic search string for a capacitor, which must only include capacitance, package, and minimum voltage. For example, 1uF 0805 25V. Second, apply filters filters for the preferred search :

Choose temperature (for example, if looking for high reliability parts, choose 125C or 150C only)
Choose dielectric (for example, if you want X7R or X8R components only, enable the X7 and X8 family)
Choose tolerance (for example, if you choose 10% tolerance, then only parts with 10% will appear)
Some predefined filters are provided, for example, the radio button that says "High Reliability" will search with filters of 125C, 150C, X7, X8, COG, and all tolerances enabled. The radio button that says "Low Cost" will search with filters of 85C, X5, and 10% and 20% enabled.