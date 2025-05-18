
pub struct MyReg<'a, C: RegComms>(&'a mut MyPeripheral<C>);

impl<'a, C> MyReg<'a, C> {
    pub fn read(&self) -> Result<MyRegVal, RegCommsError> {
        // Since it's big endian, we will pad on the right while under-reading
        let mut buf = [0u8; 4];
        self.0.comms_read(0x0f, &mut buf[1:4], crate::AccessProc::Standard)?;
        let val = u32::from_be_bytes();
        Ok(MyRegVal(val))
    }

    pub fn write(&self, val: MyRegVal) -> Result<RegCommsError> {
        let outbuf = val.0.to_be_bytes();
        self.0.comms_write(0x0f, &outbuf[1:4], crate::AccessProc::Standard)?;
        Ok(())
    }
}
pub struct MyRegVal(u32);
impl MyRegVal {
    // all, even if not readable so you can see what you're trying to write
    pub fn get() -> u32 {
        self.0
    }

    // write
    pub fn zeroed() -> Self {
        Self(0)
    }

    // resettable
    pub fn reset_val() -> Self {
        Self(0x73)
    }

    pub fn toasty_bit<'a>(&'a mut self) -> ToastyBit<'a> {
        ToastyBit(self)
    }

    pub fn n_toasted<'a>(&'a mut self) -> NToasted<'a> {
        NToasted(self)
    }
}

pub struct ToastyBit<'a>(&'a mut MyRegVal);
impl<'a> ToastyBit<'a> {
    // If read
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    // If write
    pub fn assign(self, val: bool) -> &'a mut MyRegVal {
        self.0.0 &= !(!val as u32 << 2);
        self.0
    }
    pub fn set_bit(self) -> &'a mut MyRegVal {
        self.assign(true)
    }

    pub fn clear_bit(self) -> &'a mut MyRegVal {
        self.assign(false)
    }
}

pub struct NToasted<'a>(&'a mut MyRegVal);
impl<'a> NToasted<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 4) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut MyRegVal { // [5:4]
        self.0.0 &= !(!(!0 << 2) << 4);
        self.0.0 |= (val as u32 & !(!0 << 2)) << 4;
        self.0
    }
}
