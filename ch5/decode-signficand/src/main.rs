fn main() {
    let n: f32 = 33980.912;
    {
        let n_bits: u32 = n.to_bits();
        let mut significand: f32 = 1.0;

        let exponent = ((n_bits >> 23) & 0xff) as i32 - 127;

        for i in 0..23 {
            let mask = 1 << i;
            let one_at_bit_i = n_bits & mask;
            if one_at_bit_i != 0 {
                let i_ = i as f32;
                let weight = 2_f32.powf(i_ - 23.0);
                significand += weight;
            }
        }
        println!("significand: {}", significand);
        println!("exponent: {}", exponent);
        println!("{}", significand * 2_f32.powf(exponent as f32));
    }
    {
        let n_bits: u32 = n.to_bits();
        
        // 符号部 (1ビット)
        let sign = if (n_bits >> 31) & 1 == 1 { -1.0 } else { 1.0 };
        
        // 指数部 (8ビット), バイアス値は127
        let exponent = ((n_bits >> 23) & 0xff) as i32 - 127;
        
        // 仮数部 (23ビット) + 暗黙の1を含む
        let significand_bits = n_bits & 0x7fffff;
        let significand = 1.0 + (significand_bits as f32) / (1 << 23) as f32;
        
        // 値を計算: sign * significand * 2^exponent
        let result = sign * significand * 2_f32.powi(exponent);

        println!("significand: {}", significand);
        println!("exponent: {}", exponent);
        println!("{}", result);  // 出力: 42.419998   
    }
} 
