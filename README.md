# Solana-Event-Ticketing-System
## Genel Bakış
Etkinlik Biletleme Sistemi, Solana Devnet üzerinde dağıtılmış bir blok zinciri tabanlı uygulamadır. Etkinlik yönetimi ve bilet işlemlerini güvenli ve verimli bir şekilde sağlar.

## Özellikler
- Etkinlik oluşturma ve yönetme
- NFT bilet mint etme
- Bilet satın alma ve transfer etme
- Biletleri yeniden satışa çıkarma

## Kurulum

### Ön Koşullar
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) installed
- [Anchor](https://project-serum.github.io/anchor/getting-started.html) installed


### Clone the Repository
```bash
git clone https://github.com/mrtozy/Solana-Event-Ticketing-System.git
cd Solana-Event-Ticketing-System

##  Proje Detayları:
Etkinlik Biletleme Sistemi, Solana blok zincirinde dağıtılmış bir uygulamadır ve etkinlik yönetimi ile bilet satışlarını kolaylaştırır. Kullanıcılar, etkinlik oluşturabilir, biletleri NFT olarak mint edebilir ve bilet alım-satım işlemlerini merkeziyetsiz bir çerçevede gerçekleştirebilir. Sistem, Solana'nın ölçeklenebilirliğinden yararlanarak sorunsuz ve düşük maliyetli bir biletleme deneyimi sunar.

## Vizyon:
Amacımız, blok zinciri teknolojisini kullanarak etkinlik biletleme sistemini yeniden tanımlamaktır. Solana blok zincirinde güvenli, şeffaf ve verimli bir sistem sağlayarak dolandırıcılığı azaltmak, aracıları ortadan kaldırmak ve işlem maliyetlerini düşürmek istiyoruz. Bu projeyle, dijital biletlemede yeni bir standart oluşturmayı ve herkes için daha erişilebilir ve güvenilir hale getirmeyi hedefliyoruz.

## Proje Açıklaması:
Etkinlik Biletleme Sistemi, Solana Devnet üzerinde dağıtılmış bir blok zinciri tabanlı platformdur ve etkinlik yönetimi ile bilet işlemlerini kolaylaştırır. Kullanıcılar etkinlik oluşturabilir, NFT olarak bilet mint edebilir ve bilet satışlarını merkeziyetsiz bir çerçevede gerçekleştirebilir. Sistem, düşük maliyetli ve yüksek hızlı işlemler sunarak güvenli ve şeffaf bir biletleme deneyimi sağlar. Ana özellikler etkinlik oluşturma, bilet mint etme, satın alma, transfer ve yeniden satışa sunma işlemlerini içerir. Proje, modern biletleme ihtiyaçları için verimli ve ölçeklenebilir bir çözüm sunmayı amaçlamaktadır.

## Vizyon Açıklaması:
Blok zinciri teknolojisini etkinlik biletleme sistemini yeniden tanımlamak için kullanmayı amaçlıyoruz. Solana blok zincirinde güvenli, şeffaf ve verimli bir sistem sunarak dolandırıcılığı azaltmayı, aracıları ortadan kaldırmayı ve işlem maliyetlerini düşürmeyi hedefliyoruz. Solana'nın kapasitelerinden faydalanarak güçlü ve ölçeklenebilir bir çözüm oluşturmak ve dijital biletlemede yeni bir standart belirlemek istiyoruz. Bu, etkinlik düzenleyicileri ve katılımcılar için daha erişilebilir ve güvenilir bir deneyim sağlayacaktır.

## Yazılım Geliştirme Planı:
Akıllı Sözleşme Fonksiyonlarını Tanımlayın:

Etkinlik Oluşturma: Kullanıcıların etkinlik adı ve bilet fiyatı belirlemesine olanak tanır.
Bilet Mint Etme: NFT biletlerin belirli etkinliklerle ilişkilendirilmesini sağlar.
Bilet Satın Alma: Bilet satın alma ve ödeme transferlerini yönetir.
Bilet Transferi: Bilet sahipliğini devretmeye olanak tanır.
Bilet Listeleme: Biletleri yeniden satışa sunma işlemini sağlar.
Akıllı Sözleşme Özelliklerini Uygulayın:

Solana akıllı sözleşmeleri geliştirmek için Anchor kullanın.
Gaz ücretleri optimizasyonu yapın ve tüm fonksiyonların hatasız çalışmasını sağlayın.
Ön Uç Geliştirme:

Eğer uygulama için bir kullanıcı arayüzü varsa, temiz ve kullanıcı dostu bir UI tasarlayın.
Etkinlik oluşturma, bilet satın alma ve yönetim özelliklerini içerecek şekilde geliştirin.
Akıllı Sözleşmeleri Ön Uca Entegre Edin:

Ön uç bileşenlerini akıllı sözleşme fonksiyonlarıyla bağlayın, Solana'nın SDK'sını kullanarak.
Test ve Hata Ayıklama:

Solana Devnet üzerinde kapsamlı testler gerçekleştirin.
Herhangi bir sorunu debug edin ve tüm işlevlerin beklenildiği gibi çalıştığından emin olun.
Dağıtım:

Akıllı sözleşmelerin ve ön ucu Solana Mainnet üzerinde dağıtın.
Canlı testler ve kullanıcı etkileşimleri için açık talimatlar sağlayın.
