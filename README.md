# rust-guess-number
I learn this book's the RUST progamming language

- [x] ส่วนของ comparing

```rust
use std::cmp::Ordering;
```
คือการเรียก standard library ที่เป็น enum ของ Ordering

cmp ย่อมาจาก comparing แล้วมี จาก pub Ordering ออกมา

```rust
impl<T: Ord> Ord for Reverse<T> {
    #[inline]
    fn cmp(&self, other: &Reverse<T>) -> Ordering {
        other.0.cmp(&self.0)
    }
}

```

- [x] ส่วนของ input/output

IO มาจาก input output
แปลว่าการจะใช้การป้อนข้อมูลใน command line จะต้องใช้ตัวนี้
```rust
       std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
```
read_line คือการอ่านข้อมูลจากinput
expect ถ้ามีค่าที่ผิดพลาดจะหยุดการทำงานของ program 

มาถึงตรงนี้ผมก็งงว่า & และ mut มันมืออะไร ใน rust โดยเริ่มต้นค่าต่างจะเปลี่ยนไม่ได้ถ้าจะกำหนดค่าตัวนั้นได้ในภายหลังต้องกำลังตัวแปลนั้นเป็น mut ส่วน & ยังไม่เข้าใจดีนักเลยสอบถาม GPT ก็ได้มาว่า

````
`&` และ `*` ใน Rust มีความหมายต่างกันเกี่ยวกับการอ้างอิงและการเข้าถึงค่า:

1. **`&`** (Reference):
   - ใช้เพื่อสร้างการอ้างอิงไปยังค่าของตัวแปรโดยไม่ต้องย้ายค่า
   - ตัวอย่าง: `let y = &x;` จะอ้างอิงไปยังตัวแปร `x`

2. **`*`** (Dereference):
   - ใช้เพื่อเข้าถึงค่าที่อยู่หลังการอ้างอิง (pointer)
   - ตัวอย่าง: `let z = *y;` จะเข้าถึงค่าที่ `y` อ้างอิง (ซึ่งก็คือ `x`)

สรุป: `&` ใช้สร้างการอ้างอิง และ `*` ใช้เข้าถึงค่าจากการอ้างอิง
````

- [x] ต่อเป็นส่วนของแปลงว่าที่ได้ จาก read line ที่เป็น string ( ตัวอักษรต่อกัน ) ให้กลายเป็น ตัวเลข

rust 
````
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};

````

โดยไปถามเพื่อนเก่าคนเดิม ได้คำตอบมาว่า

````
ในโค้ดที่คุณให้มา:

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

### อธิบาย:
- `guess.trim()` จะลบช่องว่าง (whitespace) ที่อยู่รอบๆ ข้อความที่ผู้ใช้ป้อนมา
- `.parse()` พยายามแปลงข้อมูลจาก `String` เป็นชนิดข้อมูล `u32`
- `match` ใช้ตรวจสอบผลลัพธ์จากการแปลง:
  - ถ้า `parse()` สำเร็จ (`Ok(num)`), จะเก็บค่าที่แปลงแล้วไว้ใน `num` และให้กับตัวแปร `guess`
  - ถ้า `parse()` ไม่สำเร็จ (`Err(_)`), จะข้ามการทำงานของลูป (ใช้ `continue`)
````

โดยหยาบๆจะประมาณนี้ ผมอาจจะเขียนเฉพาะจุดที่ตัวเองสนใจครับ XD