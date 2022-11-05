pub fn get_closest_checkpoint(chain_name: &str, height: u64) ->  Option<(u64, &'static str, &'static str)> {
    match chain_name {
        "test" => get_test_checkpoint(height),
        "main" => get_main_checkpoint(height),
        _      => None
    }
}

fn get_test_checkpoint(height: u64) ->  Option<(u64, &'static str, &'static str)> {
    let checkpoints: Vec<(u64, &str, &str)> = vec![
        (600000, "0107385846c7451480912c294b6ce1ee1feba6c2619079fd9104f6e71e4d8fe7",
                 "01690698411e3f8badea7da885e556d7aba365a797e9b20b44ac0946dced14b23c001001ab2a18a5a86aa5d77e43b69071b21770b6fe6b3c26304dcaf7f96c0bb3fed74d000186482712fa0f2e5aa2f2700c4ed49ef360820f323d34e2b447b78df5ec4dfa0401a332e89a21afb073cb1db7d6f07396b56a95e97454b9bca5a63d0ebc575d3a33000000000001c9d3564eff54ebc328eab2e4f1150c3637f4f47516f879a0cfebdf49fe7b1d5201c104705fac60a85596010e41260d07f3a64f38f37a112eaef41cd9d736edc5270145e3d4899fcd7f0f1236ae31eafb3f4b65ad6b11a17eae1729cec09bd3afa01a000000011f8322ef806eb2430dc4a7a41c1b344bea5be946efc7b4349c1c9edb14ff9d39"
        ),
        (650000, "003f7e09a357a75c3742af1b7e1189a9038a360cebb9d55e158af94a1c5aa682",
                 "010113f257f93a40e25cfc8161022f21c06fa2bc7fb03ee9f9399b3b30c636715301ef5b99706e40a19596d758bf7f4fd1b83c3054557bf7fab4801985642c317d41100001b2ad599fd7062af72bea99438dc5d8c3aa66ab52ed7dee3e066c4e762bd4e42b0001599dd114ec6c4c5774929a342d530bf109b131b48db2d20855afa9d37c92d6390000019159393c84b1bf439d142ed2c54ee8d5f7599a8b8f95e4035a75c30b0ec0fa4c0128e3a018bd08b2a98ed8b6995826f5857a9dc2777ce6af86db1ae68b01c3c53d0000000001e3ec5d790cc9acc2586fc6e9ce5aae5f5aba32d33e386165c248c4a03ec8ed670000011f8322ef806eb2430dc4a7a41c1b344bea5be946efc7b4349c1c9edb14ff9d39"
        )
    ];

    find_checkpoint(height, checkpoints)
}


fn get_main_checkpoint(height: u64) ->  Option<(u64, &'static str, &'static str)> {
    let checkpoints: Vec<(u64, &str, &str)> = vec![
        (100000, "00000440727d0569ca91813da16bb343cdab607ff29bd7fd2b4103841424b86e"
                ,"01de542845f8800bf80c889d712795097011faa487b4f72237fed1bfcb2312eb44001201c33b65eb801b8c2368f1f3ee859e884804d407a9c67bf879657f1401f676f4720000000000000000018ff64379eea39b853b31498104f7e4989f28841f04b188c8383b2bf58ca6cf6b0000000190b481ec064fad6e52ef6332e879a80d6c73e15becdbeda509701060f5452915019f5f784e9913925738f59fc12acfbde6482e2690a9e22ccf7c930c1a2b932a550000011be3837c8934c94dff21613b92d45a7dd59c68d5ab3d5d2ec07e478674a95c29"
        ),
        (150000, "00000c41ceb70bdfd1a82a58fd9bc53faf24ca91d4ce0cc8339aac3a73e79758"
                ,"015151200a2d71ef9b339963ca873ebcfd6232d3779db13b8aaa3ca371ed27091901ca9c4e44e7e8deb9968bda3b9a36e3b09a655bf2b71dcc03bbb94d4da72e3b5e120001a588629b71c025b2d4ded5b42ba4a1797d44804947910e11b83bbe77a1f34c500146574e10233b3724bc49d2bdbf3b36d5f8250da2fbaa876af92f8da03556d30c019bc91e3366651d4a550f6674dab3314d4a7ecf44294ea203147347517f5a9d67015d92a77a596060a2fb6eb972f14be43554d9775529119c28899c9c61ab153b1001929573bfe69b12e4053c015d4d9c7d680bb3d37f90ce34d48592f33a7877a64f01d934905b37d88857ae64302a6245fa3176792c7adf2db0ade0a6a1e764b5731000015641e68a44e9e92f24fa0d34a6d4c05c800a6b128473cfd4eab517dbfe40775601146dd732eabdd48f318677c949497954b915880bad0b6966a19c0a6f13a79d380001b967d08bb169212be5abc324b4df9673023d82ab5139e5c1454b6c1b8e9530070188da6bdb387a4844a09f98b7c2175b47f6a45abeae44fcd692b07e67e4afca4700000001dd392551fb2011ff0b68cff4dff53d0c28439c608777f0ced10898b0b019225a011be3837c8934c94dff21613b92d45a7dd59c68d5ab3d5d2ec07e478674a95c29"
        ),
        (200000, "0000048e814eb61aca1e36d91bea70e86ab99c3e3b3c13db17b8c4781a0ed5ed"
                ,"0100ad4feafaf58d132e9971a037765cf43b14cb8f71c1114b88e2235df9b9f50a001200000000010d268aa5c23048141f4c96786a48c1393efd212124ccfac30ebf05d7c061d36b0001e4cba4258445203cf2ad554588ad7eead7d40c531d6ea9ea94523703dfbe7c1a0001e1b7fd9d05af03d6536b4dce0b84a5a73d3fb21476b86e679ea389518c692c630169c056902ead5b9291804303e38c06cf7db5cc336d5e4b082f7af30c0d61c31e01d603c4a182cdc7647f36741c0c94d92138e30fbfff20d9f1c82d0365bb224d51010a70e4812c62bdbc42e4873ce488a88c565fac477a953649129b10c5b3b02a2201469e74bac190384ad3b1810ca37b50ee0b6d8c9531c0d17468cbb06cf857255701964f72ecac0b0d1e0fa05823fc2478ac384843eea567eb8cbf472217c2e8d93e01bef9c243f68c8483505cbb5aabb6a6bb4af982d954ecf7a37ccc24e71f8b3b370001dd392551fb2011ff0b68cff4dff53d0c28439c608777f0ced10898b0b019225a011be3837c8934c94dff21613b92d45a7dd59c68d5ab3d5d2ec07e478674a95c29"
        ),
        (250000, "0000005e81b0ce7dff9d3aa2d11f034bacfcd0328b993e7fd2356c1492dfac5e"
                ,"014c1d756996734709231f88fb7227d0f89d0b22952af646dc077d05eaf2f1bd49017c8e7800cc875b9925b6c2da36d299e6100d8dcca306cf31bbe70fec100358531200013eb42779f18424984e9370005a70a063899e3c12d3a1cc0c05568fcccfcd880a000000018566d60ff168d561ea04f44a1dda418d740478b9328d9b1fdea5bb5cee13bf2501b849876a6385f2a8029f31e252923a83112e9fa1839a40f5b2bed57e6f9163480162cd3d9e9845ca196ab1f6e34e3817ef6b99de4e4a0d48b7751768d3d01ea73d0000000001af68ddc0c532fac4f2f069768a856fe313c09474104ab660e17f414ebddd605f00010414156f3013116cdc09ffc18e6901685f4cc94c4f217a16279310dea8b3653e010de07b633a3889e70a63146fd01d02fd86a3b22571c1dce4ca0994f6af0e813701dd392551fb2011ff0b68cff4dff53d0c28439c608777f0ced10898b0b019225a011be3837c8934c94dff21613b92d45a7dd59c68d5ab3d5d2ec07e478674a95c29"
        ),
        (300000, "0000124c3aae4da7332b1e426679554bc22956ab2a98f1be0965e4da1fc98162"
                ,"0110ee26da7d5be7bf6825e4448da0870bd922a5c0f8ed23e6e3c399585af8ef66001301424bb451606d493f97005b67479288c1fce0e21e43a9bfb8c014ff5353b5780a01989cb30db6d624dc24ed437125f69786dc499772b92cff0773ea2f4ce982505701e1564554d7b08a3a17035c3789bc4ee096ac75ffc96542c575c3a19202c80001013867a898b35d35abb1285b23b6f22adbf75b1af1b8b72f375a5ef6c5364857120001673be5165aa17e35addd2bf835bac0dfde31fb0c3a47ca83cad0995298de4d380001622542899e366ee8787d5fb9c994fdb1916fcb3a02560505ccff5a9f43652d150001fef34c9e247c1a5ea772bc28aff7be95a0544f54556b45a48b0e12e4b3a8601501d823476afa720971583a4c9afed58de9858e7046d122cdf89e7d710638cbf41b00000001777d7d6c5e282fd6dd9576c83a704758ef43d1e6d1aa2002de30f1336fa0fd41000000016bd2bac2a3b188fef2d92d9fb03c0ac854493555327955705052ead280c0b305"
        ),
        (350000, "00000b5d7b20e34ef5f21c802e2d3808f8af4e0f2a2bb15de37030474e1b7b1a"
                ,"01461136d2350f96310ae55aaac5b06c577d421352ed45537dc696fca092e1f7620013000000018e706c22cfa4f64326dc3dcc89a14e03b9d84b019ac251affc06d2de76e43e0f000001d655322c588718656dbe775ee4f3c543a343805a81c131dee4c31c76a9a8254a0122a97fd6526575b97a11f6ed2c5e03cfe2aae7ce068f68d238fe9bb3cb470e0a0001b23a5d9b53d1894452eb07bd71b06474bed25289ad4e4d5297cd5f23a35d122100000128590fd0fbe55510a3f576e6b18398ec196ba162050b701fec616918668f81550001ed592e7ef34aa2b4db1ab31e995a1c2a65f9b63bf4c6c906d1ff6305122f3f5d010446002f5c9f243e940dda91f97ed3c127796137fdb20cc9ac66f3bb4e6cc20a0000016bd2bac2a3b188fef2d92d9fb03c0ac854493555327955705052ead280c0b305"
        ),
        (400000, "0000040926ae5aa842b49e899397803d556683c4bcb3d3aa1c7f62d567be0e31"
                ,"019ce99e6514edc78af5c871670d21e3f4ff4f3a1bfaaa95baa51d8a80d0fc0a49001301a7958d1ff975de39ab2fb2e8f59dde7de63bf2b1f164fb5ab34b9b5af03bfd03000162c3a4d390df65f135a5a99478f9d046a01669c56ef83a397220675ad82e365c00000001018784756690147d6e8dc8406273e2126507bfe48f8d27154c170781cb19ad5d01f965b85e72ed5a2511dc3a89a1700e97b123586f708e4a389e978a3eda274a080159e72782417cd40144ee41014fc180e16f1bdc04dcacb51386f49385f35e73390124bbe2bd6526e088fbe2a48134a41c0bfab70d8fa1d8a3d340d86290a41b0160000001cc24ca9e94ac2c3b18e5059db0b23059e05eecc7df1eaeaa923f7206542b1d180001a58ab7eed196ff752c73a2fdc71d384d2a85435a48cdfaab0dee33e52879e333000139599e48c568060d75b47e2b8131ef65ed8ba90c4f8a27119e133e4bed248e1900016bd2bac2a3b188fef2d92d9fb03c0ac854493555327955705052ead280c0b305"
        ),
        (450000, "000000cb70a7494ae6f8dab973c64f62ba81bb2eddf97bd99a1166c01db1196d"
                ,"01a38a839fab6107a2b4a4705d1147b6d8a52cb5d07711fc93e79f290aada74b6301be111e38903708c537027e2aa2bc3d22d1125e94a7e5ea986f98efe8be295119130001abe00bb6ecb9d37d5f472f341e39a83c994eab2db18c8c58c9a30a247e5b734b01612c76e35519bdb35331900dd5ec7c5eeeb73c96e6cec7e989f6943c27457a70012e1c04294d5195fa5372bb792fd321c3f7f4719cef44917a9c83a167495dd52d000000017aeacf7989b8c4a6005de192388fe618bf7cbbcade6dc16583e28a0fa9016761019ac7e2b487c52283ca37814c77fbe809f19567fe1bebe82780ef3d3231ff0719015b389bb285c5a85c98affb043071f671fb0450ebc8c7467c38046840c2dcd119000000000127bcd601c0382f98470b4b5639d442a1b38232b13d36c5c7373519f0d11ea45601343140621385f1bc51258c2692db538186b9a638f263adfd85e17851ad1337660139599e48c568060d75b47e2b8131ef65ed8ba90c4f8a27119e133e4bed248e1900016bd2bac2a3b188fef2d92d9fb03c0ac854493555327955705052ead280c0b305"
        ),
        (500000, "00000aedfdfaa17e710f81e236487fddf8462f6dd1301d3c3ec9d169a6ba1708"
                ,"0110120f7ae17a4a2c13f676713eb36af9117470ab2a7086bfec59b4c974812b1800130000000001ce959c030e6224b8277452d06d3a7c22d3b6be78d3a104679f71abf2083e005f01f46a9ba0d66457bac8d6fd69a619f735228a5e48a816551b1275739aab28042001dbb7a5f226f10e8725c5b2c860e12b0f87067d02695a4e5b69eb1f64518de1440001124625d5255e3093a3ac06a4b8e496249bbda1aa46383e7c1238c354a8bd2c6f0144d9fdfa762c0a3c4fb2865dc0998762d0b59d530de253a4dbf4b43b1f2b1811000001ebf82a2f185f69a856817798110b86598a10ac3c5cf688c365b78203638bdd0f01e5de25ff095ab05bf0d5119f74b926a4a23c0d381cf14059915b836ca95425220000000195eab1bea8465f9206ceeb6b6f2c4f56e93135a83c29f8b3f075de462787a71c016bd2bac2a3b188fef2d92d9fb03c0ac854493555327955705052ead280c0b305"
        ),
        (550000, "000007cdddedecd4e7d4a7700c8edb4eae0fc9fcee0c6d4a041ada08007c9ec1"
                ,"01866d85c32c55b8aa9a1d497e7c1e57b998bddd6f62951fc6b00b0733a71d6b180117e8251c1592df7b4c6a76f090bb96f59df86ddd41fc8460b83ae257388263621301e97d813b4c349da029a65c7d945ee5863f44019dce1016d25281d96a6c0c443e00000126073b4dc208c691f9b9c691179798799de347b2f2a45bd54eeb1b36f310390800000130e0289192651767f12f21484937f9815a1a2648b12907bd9122adb2449bf70d018673667d55ed084639b982d53d5e7959123ac8bc06e096f51a32beca043d9b2c0001d02a48cf8f337075172d238b5cec8d404b9b924148d104c3a24817f5e5cd8d1501b8d86762168fdafcef733a3403fa917272aa6bf4a6f9a09d28e5c1d7d8f6dc1a000175224d8916a28cec0671c1de66716055f8801d1d0960535c34b080b6138fad420153c1f24e0174085a3b9011eb24c262d4f62325d3c016e6bd8d2e9c77114c00150173c69d79ef4446307562016d149c847b11a27767d3633890a7f6f2c65489285400000195eab1bea8465f9206ceeb6b6f2c4f56e93135a83c29f8b3f075de462787a71c016bd2bac2a3b188fef2d92d9fb03c0ac854493555327955705052ead280c0b305"
        ),
        (600000, "000005ae4a0aee2ad937dc2880bd8a4147060920b7ae8e12e805252f0c5a035b"
                ,"01d1e6cfde67bcea4f25efb19ba11a87bdb4ab087d2340f9c600cd54ec45c6c10900130173c74ae393678399d3b442d9e236ce4219108bdaaf57cc2ef5cbf245af271542000000000152a6c324540dfd3796708df6daccac33f5d41f036e93e18c3486add84a80707300000153bed188faad18989948d3e70e34f4778e68d8583b27b3f587fa0530208f0a48010e2155b7a9534aac47e3f33d8fa205725c4c61fb2e2dcb20f058acac9d0c871301db29d7e726bff3b17535e7ef52be51d134e843129ee3c1ff79e2ac3dd0ac6e480166d42b05ffe37f25ed7d2537421eff6f1e89b3d9083150714cd2bebb084da41e0001fef5ec2db8b515a5ee9aceba979102b94102b11dce86af73b6c59e98bfec07000001626713a3ca0cc0a85acc0453abb0f841ae91da0852801d217b1a06e38fc98050000195eab1bea8465f9206ceeb6b6f2c4f56e93135a83c29f8b3f075de462787a71c016bd2bac2a3b188fef2d92d9fb03c0ac854493555327955705052ead280c0b305"
        ),
        (650000, "00000a34d39cbb99f2205d5ccbe83b192f53f019f775d32b6b9e9dc6ff7f8a95"
                ,"0107c6608dda434d4e989e5a92fe6e44df41a17b2931cfdb1f05896c749a988c48011f6c6656b6036e26e9f9aaae88224dfacc66b59de6954a6612ad74fc153e274613017b3a6487e2401fb90cceed7606fc3178e59f1da74f8ffd67168f9adcce867c520144115a469c266bccdca251ea52a417b65d79ff72a97ba777f2ed88b63e512828015019f977d87386c28c19377fdcfcfe25145f65a2176a2fd1626bdb7094259e5501710c957fcd76503f410d16620e9a979c015acfc70c7df75241c8a9ca451d652501fb289cccdfd07b662705cbcb1e44210455211ea970fbfa9998895dd055534b14017bb3040e6f9a20ad60dfaa3538c07e7725344db3dec4ccc582cf8f5c28109441017016c07a03b71c27ef724de1525fb289b0b166c2ea5c0c9e0439480e73bc111801d9ce7ab65e5f093a894ef9c6d3d400b883b0d1aeb149f87ab8036567ca80666700013b5b6757faec81a443e12a430091d7191452dbf1bd05b7d7c8820403b0c5b32c01476e3662b849a0a18a83319c4a1587cc738c46a2d47b969321326cca3088fa5501880f7fc2496f56a5305b362f2c8aabf32778f20d1011bda1abdf03edc7b6af0a0152427b908928758d42adf07a3de63522c840f9e8800608f3c618bdda6ae4cd3400014ff19d856f7a12ee7857c024febf910a4d3a345d986a80dfbbc12d34078beb2001626713a3ca0cc0a85acc0453abb0f841ae91da0852801d217b1a06e38fc98050000195eab1bea8465f9206ceeb6b6f2c4f56e93135a83c29f8b3f075de462787a71c016bd2bac2a3b188fef2d92d9fb03c0ac854493555327955705052ead280c0b305"
        ),
        (700000, "00000fbe2c3fab45fd721dd917e1bcbbcb2955dab8b952fd0fb336d7f6d5d07d"
                ,"016bd603a841e7399b0c62baa12c5110a669b90c7ab62e105a1724f75583ff404e019c4830f4f077de41a0f306d7e2f20ee79eed2a2dd2e810cf0b8681d2eba16341130194b241d58012448481ff64220e9343955f9c86ca97447bac63df64fed5f3a40c0001ac4b08c2897005c6f0a9a88d458a4ed3ba1246e1b2b644c05b04089666c9db6e0000000179d712d869c826202146a4cd3be8db841d98811fcb20eecda58ffc2ecfd9ff2a01ce32489d8bbb115acc692cababeac4524889e49a053ba2442e34fab800747e1b0000000139465362fe215a84e9a3250c6e41ad3993aa97ff6b1862ca916035e07102576b0000000001deb6b753217b1334a197aa04f8cefb2d0ad451b935974a8fd085e38c4627c00e0195eab1bea8465f9206ceeb6b6f2c4f56e93135a83c29f8b3f075de462787a71c016bd2bac2a3b188fef2d92d9fb03c0ac854493555327955705052ead280c0b305"
        ),
        (750000, "00000f3c4642e7a989cc9157f94ec517798de934e785e7faad5b5c0f370808f1"
                ,"013999c8f027cd56173b9350116cb1074f302be34b2cab01173f026db3b009845a01db16706b2dc87538d980cfc741b493c9fc2c1df4c4b71cb52920f87258d2a46f1300012bde89a922506a4cb8812c1d4fd6f354acc3285c210c235359b17cf6dec1c43d0182e0211307891fb8e5643240d37f99c0893d021d777711fd139c5780a7dabf6d00019846229061ba225f46945e2378ab52f575ac35ea76f29b2d6cec393ae76b264601980fbd9a872b80582668b05c4013504c07e666684575564c0a1a6be0e13c9405000001aee5fc53534fcba010ec5b1f994754bb732fe7c046a2a853e474cb5e898dbb0e0128cce7d7e8ade61a1112eedd49ed00bd6e35ef627b27a867a2a7a77f81e2423f0001cbcf0503220ea54de6ab90a66c8acbe4c1832e3af885fceebe6d54e3f7161758000185d4b6ba8adb1a4553d0c307871d5f9cab973ab884c852ffa1c35dd86949c450000001deb6b753217b1334a197aa04f8cefb2d0ad451b935974a8fd085e38c4627c00e0195eab1bea8465f9206ceeb6b6f2c4f56e93135a83c29f8b3f075de462787a71c016bd2bac2a3b188fef2d92d9fb03c0ac854493555327955705052ead280c0b305"
        ),
        (800000, "00000426f78268a4e71cbc54705609cb19a9e8217af105cf06269ebe559244d3"
                ,"0164ce05c53926ac410b019e3a082ca90e3619cfc336eced86516619ebf5075d1b011413916a638275c9322586c967b2f959ad4f159002e40c9db2645cc4adea1854130000019949e2984fc6f4db0c2e9dd20599533a971c19310df8c2b81da2e120a578b61000018d47e18cc9e7ed7d87e4f2bfe51d0bb2f86a2d824845ce3a0ea5b77d7b6e37180000000158ed4932d0bb11fb2f0d7fde9f2b67b9062377b5e27e89a011f00b13b5f7ab7101e0df4b8889cb5b5bd1ee044167ab38713fe5c33db974a7aab83abc33a650f54300000171b00058eb87a903b05fbe2e0a21def3e7b91a493b4f82cf3a6182bf7600981f000122a958c261b7782ffc879b6dafc0ebcd84e4063e1a734bf1ce0233d2b2e553570001deb6b753217b1334a197aa04f8cefb2d0ad451b935974a8fd085e38c4627c00e0195eab1bea8465f9206ceeb6b6f2c4f56e93135a83c29f8b3f075de462787a71c016bd2bac2a3b188fef2d92d9fb03c0ac854493555327955705052ead280c0b305"
        ),
        (850000, "00001837f59fd751dad63ec382cc549c3c63725046529b388d7d7af7844e50c4"
                ,"018968deb1dc9003ead11584d82e308e672725644107469d04aa71911f8995533000130001434b97de34b1c0e96993f6f136962ef8c611939ef620b86c5a7183de81d668480148559d39cec65e00f3fd310cd7388d2044cb972b30fc76dd25c45473ec35610c0000019d65a9762b919b87347ad44c542cea50f9eb91337915cf0d6b2ef45845b6c21b0196d2f50860dc2df4fb14b55492267721f554d089c0dfbb34de7c3e272985d80700010bd48b25b8b2de15214898bd6ac001aaa785dd182867476820e8284fc03c0c2700000001df72bb33092660311ae3d77b69d91e76b70d982d0e0351fca55dc8e57d55ac3401cab32fa87a2c6cd19bd48198c08b2f97debcb21cf9f4ac1eed621fc1fd6c1a0b0122a958c261b7782ffc879b6dafc0ebcd84e4063e1a734bf1ce0233d2b2e553570001deb6b753217b1334a197aa04f8cefb2d0ad451b935974a8fd085e38c4627c00e0195eab1bea8465f9206ceeb6b6f2c4f56e93135a83c29f8b3f075de462787a71c016bd2bac2a3b188fef2d92d9fb03c0ac854493555327955705052ead280c0b305"
        ),
        (900000, "00004e0469ab4872c0340311328a07e0dd53e939e74865cdd5692208b93e1b01"
                ,"01e8e5ee3edeaa60bcac523df4b5ffa175da1e6d38ca7d27107e9530ef9d057c6601226285037f3b5a181e285fd89ceb912f7f0303c8c5a04ef5a460466407ecea051301b217f0c8f09cb2e4682ae38c8d143670d6deb5e66158d1177ec10dc912ae3b0d0165a05586e1c5a28201c7ad13bb8da92dffc73368d2d4c593485260c691ec143d0001d28f693fc5654a450f1e98f83c7d97343c66ea11df6a0946ac07db3b7e230d64000001a8e4c0a8e1a7efa1542941461902b4c28b0433fd2858984b6adbadd0937688500000010a7917bca84f349f5b8f9e39680eff35f3f6c3733987d2d74cd8a377bc09fb15000199dcb0b23598b3a191c63ac18012fa81f9ded5f443bba56189e778617e062e59000000019ac7d93d5b1b99a0d4a5071617df07754d16add59c921c8bc93d5932bb934d4401deb6b753217b1334a197aa04f8cefb2d0ad451b935974a8fd085e38c4627c00e0195eab1bea8465f9206ceeb6b6f2c4f56e93135a83c29f8b3f075de462787a71c016bd2bac2a3b188fef2d92d9fb03c0ac854493555327955705052ead280c0b305"
        ),
        (950000, "00001062c4520df10e7bac9263d4e203e9d2f72a6c767ed2070b8fc12e8a47bf"
                ,"0152f1c68701e6b8a73c1d9229deac0118e6e39e29b7ff9ed56be84e53863cff1701a32331808fe200bc94e16cf8bc1f1bb4e65e6a17f7deb96f069a8267177e2156130195a82927092e7d4b4f8f102bfe8b923a859ae1505401d81750067e896ad85154000001a8812cef938c633901f1d2b84a33e35a51e9a671e7868fb80e858835b573cd260000000001ab15bceeb9cc380bebb9922cb126c27d31ef7193311f4287dcdb75e8d87ce446017284a89f995dd27265a589092464fd87b080ce69cf493e28357abc6b94fe2b0200000001cc4fcc2327cdc444c90b2543e47ab23c733eac390039836e4a663e5619e8e95300019ac7d93d5b1b99a0d4a5071617df07754d16add59c921c8bc93d5932bb934d4401deb6b753217b1334a197aa04f8cefb2d0ad451b935974a8fd085e38c4627c00e0195eab1bea8465f9206ceeb6b6f2c4f56e93135a83c29f8b3f075de462787a71c016bd2bac2a3b188fef2d92d9fb03c0ac854493555327955705052ead280c0b305"
        ),
        (1000000, "0000e78b82c4eacea4ba5cf3ebaf2d2b5e597cb08af7dba41935899e0c922179"
                ,"01128928a7322b191c0f1ae6e108d00e81ca266059623a29f384be54631883484300130001ed4b801e798275f7058292bd9ea8609baa5a83a45470a7ab8c8ffbd0d72c281700010b4366df0c5f943f4f382b3d003472840c8bd42568b88f1366e7a8223a73e60700000001ec242f7322b2f33594e0832eaa8dd438fa94d4659e5fae5a113e6d2b17e46e5101e2c523e91b891fa474d19e3e1a272030de5709173c35a27dc10bddbfa0044d560000000000012b15609c1ad9042a6ac206be67d9cabf8df22378f715557d6a794d875262fa54019ac7d93d5b1b99a0d4a5071617df07754d16add59c921c8bc93d5932bb934d4401deb6b753217b1334a197aa04f8cefb2d0ad451b935974a8fd085e38c4627c00e0195eab1bea8465f9206ceeb6b6f2c4f56e93135a83c29f8b3f075de462787a71c016bd2bac2a3b188fef2d92d9fb03c0ac854493555327955705052ead280c0b305"
        ),
        (1050000, "0000295bf55a2858875a6dcd223e95a7f5012553b1367bbe356aad40a6bebef4"
                ,"01113ae3ec219b433d58852503b6d959f3fbb1b9d0adab7a99dd26aba35f67e40101e4f9d76edb689e4df2feaac51a49dbfa6442d249f942722a59df5816d47d3425130001c79fa87c9aa503f395334f4a99d48a8901d9baa98e6e26c5e68379310304e50d0142b077d88a2c7dfb267a6d634cbbc35e3f7cc6dd598a408dd0908357ac2e010b00016e7f5cfa076548eb4fb88a165b0c197eb08a8bd3f473440d41087d1ca807a00201ce18a1d20084a86a95735b13be5c463a5f913ac7dd564c7746e3b9aa08491a6f000001fc02b7b90f7c3a666c22ee99c7c3e8788c3dd3f43c1f9801e42a682869f45b250001e239bbf6097db003c04627a83df95af341c95abfc29c794d1670273b180708260172ed1c75e02fb083f8a59d4f7e5fb4ff2d6026a6916c906450007061a39fd1310000012b15609c1ad9042a6ac206be67d9cabf8df22378f715557d6a794d875262fa54019ac7d93d5b1b99a0d4a5071617df07754d16add59c921c8bc93d5932bb934d4401deb6b753217b1334a197aa04f8cefb2d0ad451b935974a8fd085e38c4627c00e0195eab1bea8465f9206ceeb6b6f2c4f56e93135a83c29f8b3f075de462787a71c016bd2bac2a3b188fef2d92d9fb03c0ac854493555327955705052ead280c0b305"
        ),
        (1100000, "00023f726e949f47326d2f605821ef00dce880da53083c847a9f852665bde1d8"
                ,"0123e7ea2f800dff29969bf34082835ab6b2b1f3f2b1e9ba0a77e277aafe03270d00130000000157d74ac10a51dd4e890d73cf3e2f5a7e6d8a3f9fffdc5a232ec81dae2ffbfe4401c469ecf8a17445372516fc107c0e131ab3e8e675e1023e33cffc2e6ef6a1311801d44c57da4a646f15d548dd1db6a350105d77907391e0132e7b50797374040401013866b408a2bb132690fac4e6a0428dac51a50fce148f3339ecbe39b5276e853800000190c24969f330fb857a87155806545ed2a2199bf9815f2739eb2780adb1272632019a15ca251107fcac19d35a646a0aa79b0767af9b7ab1adef3c16e115a68f4d4d0001a88bc771c07aeda856b886d7c81b8c40bb9908e5407ddc79233b7f8eed4e2d2200012b15609c1ad9042a6ac206be67d9cabf8df22378f715557d6a794d875262fa54019ac7d93d5b1b99a0d4a5071617df07754d16add59c921c8bc93d5932bb934d4401deb6b753217b1334a197aa04f8cefb2d0ad451b935974a8fd085e38c4627c00e0195eab1bea8465f9206ceeb6b6f2c4f56e93135a83c29f8b3f075de462787a71c016bd2bac2a3b188fef2d92d9fb03c0ac854493555327955705052ead280c0b305"
        ),
        (1150000, "000185c1d51bff9efb7d2dc5909047845d6b1fd2429d3c63916cab6cf49517fa"
                ,"018bf98c7ebec3191bdf783ede40b4294056cda1ab4cb420557cb237c1e3970765018bf2091834ff97e60472dcb794ac6df342a89491ec9729eb28e079912b69330e13010aba498b41e0da179608d9d5d83dff7d99b3fbca4459989b82df2ec674a0be1c0000017be1b91a3acdd7c0a885deed7cbec21da5a0acb5a2f0af1963b98cfc7d6704320001571057f685e26fd71135d5a1028707e03ffca58835215ded29f9d75506c03e2d0000013cda60876a73566137a97ffa8cbdaf521ced58dfe7815fcb1be0d5ad9c83b23901e4e9d59748b918f6b5dd839a8fcf71aa6c1dc13f7c4eeebe7dd56aa542e4476e000104f5248d00ecdbaa22a2b6d51475ce7e1cb1cebaf8874a29367cd15bb628014001a88bc771c07aeda856b886d7c81b8c40bb9908e5407ddc79233b7f8eed4e2d2200012b15609c1ad9042a6ac206be67d9cabf8df22378f715557d6a794d875262fa54019ac7d93d5b1b99a0d4a5071617df07754d16add59c921c8bc93d5932bb934d4401deb6b753217b1334a197aa04f8cefb2d0ad451b935974a8fd085e38c4627c00e0195eab1bea8465f9206ceeb6b6f2c4f56e93135a83c29f8b3f075de462787a71c016bd2bac2a3b188fef2d92d9fb03c0ac854493555327955705052ead280c0b305"
        ),
        (1200000, "00006d5a5af8282eb82825b17eda16940ef924f56a5fb50315a22f6cc9065401"
                ,"0102b999c5290d20c42f9e00540f8a07cbd46bbeb6159885908804e3e9a10c0a32001300016c6d1f080466b8a2a61bd229fd2c06fc41c286a7f734a021f48177d8934a9d6b01a38a6112a952426e3dc96d37c308f9ed6ca562ccccc5ab7fa0e8fec2b752e26801bc0011bc71fe2e173f6463d9a039daa68da43e1cd1c8bcc2bc7b9a1e8f8ec83b0001c2c60ef6fa0e7a46c998918463533cc89e4fc315bb7a3619ceacb5f57b54de58012e3cba615d7b38a7e6564f776714c6193479d70807032be84dd7b34a9d99ea71012529915d29a0908be0eef34dc387e05cfd8ba064a68865c68e7e2791547d262a012063966e900f9a421c92cee80ecbd04245b52604f38c58728507cf538a58d05a000135c51a73f2db171a8a1adb0c0875e511478942179d845a980d2aca47e1797b270104f5248d00ecdbaa22a2b6d51475ce7e1cb1cebaf8874a29367cd15bb628014001a88bc771c07aeda856b886d7c81b8c40bb9908e5407ddc79233b7f8eed4e2d2200012b15609c1ad9042a6ac206be67d9cabf8df22378f715557d6a794d875262fa54019ac7d93d5b1b99a0d4a5071617df07754d16add59c921c8bc93d5932bb934d4401deb6b753217b1334a197aa04f8cefb2d0ad451b935974a8fd085e38c4627c00e0195eab1bea8465f9206ceeb6b6f2c4f56e93135a83c29f8b3f075de462787a71c016bd2bac2a3b188fef2d92d9fb03c0ac854493555327955705052ead280c0b305"
        ),
        (1250000, "0001286c5cd5b102bbc7c2c7046b0555d31c13d0c42cd9cf711305a357608727"
                ,"01b5db18aac341c15b8609bd97877a0303ad95c9db2eafe1aef811b8384454502e0174cf7a3c6e47b12146d3eca09b5164f36ee99bc034d60340cd365653ce764f6913014ca8778883cc76fdbb27f75409eec7b9e7575b37c73c9f926f19aa8385578b280178eeed607642c9a97c15451f2ff3c0243c016362c10db61b9e0b5ffcada8dd6b0000019ccc4bd2a821a21edbc72b860f7c08df01816df626e6f5e51acac9cb4778af6800000000015fccdd715e694bd31d0c90437c70f38cfeddc6e6de00c6570aa7e7486b55d15500000001dd131e7be3e4ad2d2cda82449caa15522b311cb225f6a86ea41c631a7a3fa934012b15609c1ad9042a6ac206be67d9cabf8df22378f715557d6a794d875262fa54019ac7d93d5b1b99a0d4a5071617df07754d16add59c921c8bc93d5932bb934d4401deb6b753217b1334a197aa04f8cefb2d0ad451b935974a8fd085e38c4627c00e0195eab1bea8465f9206ceeb6b6f2c4f56e93135a83c29f8b3f075de462787a71c016bd2bac2a3b188fef2d92d9fb03c0ac854493555327955705052ead280c0b305"
        )
        


    ];

    find_checkpoint(height, checkpoints)
}

fn find_checkpoint(height: u64, chkpts: Vec<(u64, &'static str, &'static str)>) -> Option<(u64, &'static str, &'static str)> {
    // Find the closest checkpoint
    let mut heights = chkpts.iter().map(|(h, _, _)| *h as u64).collect::<Vec<_>>();
    heights.sort();

    match get_first_lower_than(height, heights) {
        Some(closest_height) => {
            chkpts.iter().find(|(h, _, _)| *h ==  closest_height).map(|t| *t)
        },
        None    => None
    }
}

fn get_first_lower_than(height: u64, heights: Vec<u64>) -> Option<u64> {
    // If it's before the first checkpoint, return None.
    if heights.len() == 0 || height < heights[0] {
        return None;
    }

    for (i, h) in heights.iter().enumerate() {
        if height < *h {
            return Some(heights[i-1]);
        }
    }

    return Some(*heights.last().unwrap());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_lower_than() {
        assert_eq!(get_first_lower_than( 9, vec![10, 30, 40]), None);
        assert_eq!(get_first_lower_than(10, vec![10, 30, 40]).unwrap(), 10);
        assert_eq!(get_first_lower_than(11, vec![10, 30, 40]).unwrap(), 10);
        assert_eq!(get_first_lower_than(29, vec![10, 30, 40]).unwrap(), 10);
        assert_eq!(get_first_lower_than(30, vec![10, 30, 40]).unwrap(), 30);
        assert_eq!(get_first_lower_than(40, vec![10, 30, 40]).unwrap(), 40);
        assert_eq!(get_first_lower_than(41, vec![10, 30, 40]).unwrap(), 40);
        assert_eq!(get_first_lower_than(99, vec![10, 30, 40]).unwrap(), 40);
    }

    #[test]
    fn test_checkpoints() {
        assert_eq!(get_test_checkpoint(500000), None);
        assert_eq!(get_test_checkpoint(600000).unwrap().0, 600000);
        assert_eq!(get_test_checkpoint(625000).unwrap().0, 600000);
        assert_eq!(get_test_checkpoint(650000).unwrap().0, 650000);
        assert_eq!(get_test_checkpoint(655000).unwrap().0, 650000);

        assert_eq!(get_main_checkpoint(500000), None);
        assert_eq!(get_main_checkpoint(610000).unwrap().0, 610000);
        assert_eq!(get_main_checkpoint(625000).unwrap().0, 610000);
    }

}
