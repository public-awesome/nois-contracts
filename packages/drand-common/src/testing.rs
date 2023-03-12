use cosmwasm_std::HexBinary;

/// Gets a signature from drand mainnet 2 for testing purposes.
pub fn testing_signature(round: u64) -> Option<HexBinary> {
    match round {
        // for r in {1..10}; do echo "        $r => Some(HexBinary::from_hex($(curl -sS https://api3.drand.sh/dbd506d6ef76e5f386f41c651dcb808c5bcbd75471cc4eafa3f4df7ad4e4c493/public/$r | jq .signature)).unwrap()),"; done
        1 => Some(HexBinary::from_hex("9544ddce2fdbe8688d6f5b4f98eed5d63eee3902e7e162050ac0f45905a55657714880adabe3c3096b92767d886567d0").unwrap()),
        2 => Some(HexBinary::from_hex("a050676d1a1b6ceedb5fb3281cdfe88695199971426ff003c0862460b3a72811328a07ecd53b7d57fc82bb67f35efaf1").unwrap()),
        3 => Some(HexBinary::from_hex("8176555f90d71aa49ceb37739683749491c2bab15a46094b255289ed25cf8f01cdfb1fe8bd9cd5a19eb09448a3e53186").unwrap()),
        4 => Some(HexBinary::from_hex("ae993698ec92a3da3b51a3ac0e3ef573670a3c5a279fa7858f5e13150d789a3ca302b77fab4ddf20700ba9e40ff50377").unwrap()),
        5 => Some(HexBinary::from_hex("9166529271ebe0b30ce2d021c835e7c069647ffd5d4af1d35f997371a27860679a0f9cd3b8caa888a47237016e7accef").unwrap()),
        6 => Some(HexBinary::from_hex("b5be83cc91295769734b3dc68121fc4a42945a04637ce324e79a2ce13c384e4f5989e11eca28121d18fcde739a13eea2").unwrap()),
        7 => Some(HexBinary::from_hex("94b9bd8ec15ffaf7496417e653a1dd94cee68728bd551100a0949de743f4481c820300598d8e65adca8108b1af34c0e3").unwrap()),
        8 => Some(HexBinary::from_hex("a56ec0345b0f8019a3ad85b57896d776d08ec7fe0633d7cfb2527ad723e1f43ce74b7d38fe4466385793d2cfd93ca513").unwrap()),
        9 => Some(HexBinary::from_hex("b598635395730033daab872d08c0ca31045b216fd062d911fa72f8320edad3d838f0a7b3b0013296c7bbd0f8dec01e9a").unwrap()),
        10 => Some(HexBinary::from_hex("a24c8c1b6b7ce4d01f7e61422781839209dae2b356d156517d5344f643c5f968cf0716f4c7eb105fb6a0306e88e51595").unwrap()),
        // for r in {72760..72790}; do echo "        $r => Some(HexBinary::from_hex($(curl -sS https://api3.drand.sh/dbd506d6ef76e5f386f41c651dcb808c5bcbd75471cc4eafa3f4df7ad4e4c493/public/$r | jq .signature)).unwrap()),"; done
        72760 => Some(HexBinary::from_hex("8b67bfda5e871ad79179fb8b7476852487904241c720e2434b68c51c5b9356623d242cf30d1869c9e0a642d608050c17").unwrap()),
        72761 => Some(HexBinary::from_hex("8973b77b12c8d843841b5f554c10a420483aa4896aa263f36264c6568c3c8e29828940973d579e6192429391d8cb7902").unwrap()),
        72762 => Some(HexBinary::from_hex("852cbe450aaad11990d8abffb3c7b80477d2e1afd6be2cea7f5aa9c72f37824af60e90f20c477fdd5450ec5d9442861d").unwrap()),
        72763 => Some(HexBinary::from_hex("8e4d9c68c7f0d78b060e3f3297727ebe161319b41643d011d4c6e9b9dd8a0128e5dc04520173e7a956fff8faf1b70344").unwrap()),
        72764 => Some(HexBinary::from_hex("b080d86f73971e13e8a1ad4000a1ff89494272ddc2ac5b8e33b7ec2a1c336a59f5df12e4ebdb9e9eacc7b11f626f928a").unwrap()),
        72765 => Some(HexBinary::from_hex("b6d2823946415d38aeda9dc7194e44761947ef92574fd2a48debface415951d5f3b017a86577898f0e3ca7b84081c1e5").unwrap()),
        72766 => Some(HexBinary::from_hex("b8a630a321a89bff2c3e1b68296b514166e60dbeac4ba1b1933efce5fdfbaf3081a0e1c628da6f7b267db46f9029188b").unwrap()),
        72767 => Some(HexBinary::from_hex("88285ce5ebf6fd336b78ea2fd41a69bb78617c14cc0d1fd5177076b355182df2c2139f7a6cbfeaf077e0707d8d2c5196").unwrap()),
        72768 => Some(HexBinary::from_hex("972506c3639d43efc175cea617be78ee6060a09030216b09fe752c0cbd0433196222e0facd4f47fcea31ae90934c8c88").unwrap()),
        72769 => Some(HexBinary::from_hex("b6a3513ed4f7049b54609af8ff1b971ee5ce87b505eb8e7decc48d30f1667aa7fce0745dfbd62f134a2321368d58ace8").unwrap()),
        72770 => Some(HexBinary::from_hex("85b380ce250af1e2aa9226f0f77e5ef88dafb30158bf240d745df689d002bba24db45af259bb941bfe27dd20ddc11b32").unwrap()),
        72771 => Some(HexBinary::from_hex("a056f049509fd5ef3caf03fef966a066d3c2ebfc6d415d5f481774946987f6726fe71bc7cc6bcd55636e26194565baec").unwrap()),
        72772 => Some(HexBinary::from_hex("aea4d26ab4c65baedc696f31bb3b9b304ba8520ad6810762cdd0ebed08482dcbb0e747deca7116a0f14bb7be99ab05be").unwrap()),
        72773 => Some(HexBinary::from_hex("83561198d95fae700d220869b07b10d897e2fb1ed1183713d4f174278f5e24cba97ba649ed8df671b48c8e0cc3981969").unwrap()),
        72774 => Some(HexBinary::from_hex("b70409a51ea4e70d25a4a3a78e3d289f91ef38665b89193363596545c9395eb518415a9a4bb73ad2413cb83492aaac6e").unwrap()),
        72775 => Some(HexBinary::from_hex("973ae0dd58e53c7ca80952ee26e0565627dd61cc5ded60b20d2d846e5354d2aec13d08a2bfbc240c794993d16a0dae90").unwrap()),
        72776 => Some(HexBinary::from_hex("9191b08e1f9c0701d66634e8bd4ded33fecb428e96d0d33b17454001c1fcb41857d7d59695bc184eaa828c536eba4e26").unwrap()),
        72777 => Some(HexBinary::from_hex("aa64226aae1afe27835bf90f08b42ce5f8c239a37a542b5eb1a11549f8083c98b5c179488bf04bde951fffd7e1f6440e").unwrap()),
        72778 => Some(HexBinary::from_hex("b88ab76e57d12bac286df63dd45f0360a8f88a79986a1a885ab7cffd1cd54f790fee3ccb93eb75f119501db9a07d8c76").unwrap()),
        72779 => Some(HexBinary::from_hex("983b9c63b1b22f82397bdb13dc790cd5d89bdc09f21e48b983b8408899012b9967d02dcd604f1e0e394bb2f98498c993").unwrap()),
        72780 => Some(HexBinary::from_hex("86ac005aaffa5e9de34b558c470a111c862e976922e8da34f9dce1a78507dbd53badd554862bc54bd8e44f44ddd8b100").unwrap()),
        72781 => Some(HexBinary::from_hex("9321b3f2139b488f0d4f7c2628a8fa6f3af24a0a405b83946eb5a0c4e659f09dc220a8a938c82f2da73f870632a7a268").unwrap()),
        72782 => Some(HexBinary::from_hex("98849d18cd21ec6d9aadb87c12a628a8cfc1160e9dac6f88b0496414cac5fa9d594438da8f5321f5438e3c3ab71a5d33").unwrap()),
        72783 => Some(HexBinary::from_hex("a2e3c79477d6bbb67ff9dcc78acd53c223c3dea35c6b909aae79b1ad591b8e3aaa53f47811c1e1c00877756c6ba2438c").unwrap()),
        72784 => Some(HexBinary::from_hex("b9555d15e77ea0b9e23b976851761dac830de0576a50faa39c1c4b3d097d5568a979a7cae0e95997b82f15272fd37fe8").unwrap()),
        72785 => Some(HexBinary::from_hex("83f2bcb12b772602f27a1ad130a33781014ac73e82098580e934a5b5e4ad57ceff27ad22fd6344b33af9675e0d0b5e27").unwrap()),
        72786 => Some(HexBinary::from_hex("845fa920aa818c5417e1f7be321f428d363f21acdede449bec2af7be7fdd0c26203829e4063703101a19770a3767d31c").unwrap()),
        72787 => Some(HexBinary::from_hex("a989c38f3a9e9b871059f2dab78ea5db546196d00fc94db2893de609f3408f73adadc345e21df15d9e351bda365ebe0f").unwrap()),
        72788 => Some(HexBinary::from_hex("b90c4026361eedafb45e93b5cef36e54cc0d7609838d0a7e99e96c0de04361a536cb119925a2bceb28d9bca361ded5a1").unwrap()),
        72789 => Some(HexBinary::from_hex("b28d0686ce6a2ba23b4cd9a5a008d262facbe34546cd19b9f5bf35bc62eec9b5eee629bec3bf467362577bdb39710220").unwrap()),
        72790 => Some(HexBinary::from_hex("b366eaf8ad13fbd36b76751ded49726f8cc47506af50c4e12d9fd1c3244418e0ac4c36d5cdcc342fa71e5bdb599564c9").unwrap()),
        _ => None,
    }
}
