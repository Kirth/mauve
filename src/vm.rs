type Register = u32;
type OP = u8;

struct MMVM
{
    pc:     Register,
    sp:     Register,
    a:      Register,
    b:      Register,
    memory: [u8; 2048]
}

impl MMVM
{

}