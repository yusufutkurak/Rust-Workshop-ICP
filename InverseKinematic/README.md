# Inverse Kinematic Calculator - ICP 
Bu Rust programında robotikte kullanılan ters kinematik hesaplamaları yapmak için ICP(Internet Computer Protocol) altyapısının özelliklerinden yararlanarak ,basit bir `InverseKinematic` Canister algoritması yazdım. 
Inverse kinematic model kısaca, eklemli robotlarda eklemlerin açılarının, uzuvların gidecekleri konuma göre matematiksel yöntemler kullanılarak hesaplanmasıdır.

## `InverseKinematic` Yapısı 
`InverseKinematic` yapısı, bir robot bacağının kinematik özelliklerini temsil eder. 
Bu yapı aşağıdaki alanlara sahiptir: 
- `tibia_length`: Tibia (kaval kemiği) uzunluğu
- `tibia_angle`: Tibia açısı
- `femur_length`: Femur (uyluk kemiği) uzunluğu
- `femur_angle`: Femur açısı Verilen parametrelere göre atak ucunun gideceği konumu matematiksel hesaplamalar yardımı ile hesaplar

