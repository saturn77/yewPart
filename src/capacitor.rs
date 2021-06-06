


    #[derive(Debug)]
    pub struct Dielectric {
        pub x5  : bool,
        pub x6  : bool,
        pub x7  : bool,
        pub x8  : bool,
        pub cog : bool,
    }
    
    impl Dielectric {
        pub fn new() -> Dielectric {
            Dielectric {
                x5  : false,
                x6  : false,
                x7  : false, 
                x8  : false, 
                cog : false, 
            }
        }
    
        pub fn clear(&mut self)   {
            self.x5  = false; 
            self.x6  = false;
            self.x7  = false;
            self.x8  = false; 
            self.cog = false; 
        }
    }


    #[derive(Debug)]
    pub struct Temperature {
        pub t85 : bool,
        pub t105 : bool,
        pub t125 : bool,
        pub t150 : bool,
    }
    
    impl Temperature {
        pub fn new() -> Temperature {
            Temperature {
                t85  : false,
                t105 : false,
                t125 : false, 
                t150 : false,  
            }
        }
    
        pub fn clear(&mut self)  {
            self.t85  = false; 
            self.t105  = false;
            self.t125  = false;
            self.t150  = false; 
        }
    }

    #[derive(Debug)]
    pub struct Percent {
        pub p1 : bool,
        pub p2 : bool,
        pub p5 : bool,
        pub p10 : bool,
        pub p20 : bool,
    }
    
    impl Percent {
        pub fn new() -> Percent {
            Percent {
                p1  : false,
                p2  : false,
                p5  : false, 
                p10 : false,
                p20 : false,  
            }
        }
    
        pub fn clear(&mut self)  {
            self.p1   = false; 
            self.p2   = false;
            self.p5   = false;
            self.p10  = false;
            self.p20  = false; 
        }
    }




