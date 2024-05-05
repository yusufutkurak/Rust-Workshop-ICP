use candid::{CandidType};

#[derive(Debug, CandidType)]
struct InverseKinematic {
    tibbia_length: f32,
    tibia_angle: f32,
    femur_length: f32,
    femur_angle: f32
}

impl InverseKinematic {
    fn new(tibbia_length: f32, tibia_angle: f32, femur_length: f32, femur_angle: f32) -> Self {
        Self { tibbia_length, tibia_angle, femur_length, femur_angle }
    }
    
    fn calculate_angle(&mut self, x: f32, y: f32) {
       
        let hypotenuse = (x.powf(2.0) + y.powf(2.0)).sqrt();

        let tib_angle = 
        ((self.tibbia_length.powf(2.0) + self.femur_length.powf(2.0) - hypotenuse.powf(2.0))
        / (2.0 * self.tibbia_length * self.femur_length)).acos().to_degrees();
        
        self.tibia_angle = tib_angle;

        let fem_angle_cos = (self.tibbia_length.powf(2.0) + hypotenuse.powf(2.0) - self.femur_length.powf(2.0))
        / (2.0 * self.tibbia_length * hypotenuse);
        self.femur_angle = fem_angle_cos.acos().to_degrees();
    }
}



#[ic_cdk_macros::query]
fn get_angle(tl: f32, fl: f32, x: f32, y: f32)-> InverseKinematic{

    let mut leg = InverseKinematic::new(tl, 0.0, fl, 0.0);
    leg.calculate_angle(x, y);
    
    leg
}


