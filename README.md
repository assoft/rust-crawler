# rust-crawler
Son Depremler ve Vakit Namazları

## Vakit Namazları

`fetch_today_prayer_times()` Belirtilen ilin koduna göre (il kodları için diyanet sitesine bakınız: https://namazvakitleri.diyanet.gov.tr/) günlük namaz vakitlerini çeker.

## Son depremler

`fetch_latest_quakes()` Boğaziçi Üniversitesi Kandilli Rasathanesi websitesinden (http://www.koeri.boun.edu.tr/scripts/lst6.asp) son depremleri çeker. Veriler JSON formatında dışa aktarılabilir.
