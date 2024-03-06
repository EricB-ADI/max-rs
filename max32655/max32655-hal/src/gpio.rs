use max32655_pac::CorePeripherals;






pub struct Port0;
pub struct Port1;
pub struct Port2;



enum PortError{
    InavlidPin
}

impl Port0 {
    pub fn read_pin(self, pin_num :u8) -> Result<bool, PortError>{
        
        if pin_num > 31{
            return Err(PortError::InavlidPin);
        }

        let res = (crate::pac::gpio0::IN::read(&self).bits() & (1 << pin_num) )!= 0;

        return Ok(res);

    }

}
