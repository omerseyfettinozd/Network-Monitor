import tkinter as tk
import psutil
import time

# Colors / Renkler
BG_COLOR = "#33240D"          # Deep Brown / Koyu Kahve
TEXT_MAIN = "#CAC2B3"         # Light Beige / Açık Bej
TEXT_HIGHLIGHT = "#F4DAB0"    # Cream-Light Orange / Krem-Açık Turuncu
ACCENT_COLOR = "#756244"      # Medium Brown / Orta Kahve

class NetworkMonitorApp:
    def __init__(self, root):
        self.root = root
        
        # Window configuration / Pencere yapılandırması
        self.root.overrideredirect(True)  # Remove title bar / Başlık çubuğunu kaldır
        self.root.attributes('-topmost', True)  # Always on top / Her zaman üstte
        self.root.attributes('-alpha', 0.9)  # Transparency / Saydamlık
        
        # Transparent key for rounded corners / Yuvarlak köşeler için saydamlık anahtarı
        self.transparent_key = "#000001"  # Almost black, unlikely to be used / Neredeyse siyah, kullanılma ihtimali düşük
        self.root.configure(bg=self.transparent_key)
        self.root.wm_attributes("-transparentcolor", self.transparent_key)
        
        # Set window size and position / Pencere boyutu ve konumu
        self.width = 320
        self.height = 180
        screen_width = self.root.winfo_screenwidth()
        screen_height = self.root.winfo_screenheight()
        x_cordinate = int((screen_width/2) - (self.width/2))
        y_cordinate = int((screen_height/2) - (self.height/2))
        self.root.geometry("{}x{}+{}+{}".format(self.width, self.height, x_cordinate, y_cordinate))

        # Initial Network Stats / Başlangıç Ağ İstatistikleri
        self.last_upload = psutil.net_io_counters().bytes_sent
        self.last_download = psutil.net_io_counters().bytes_recv
        self.start_upload_total = self.last_upload
        self.start_download_total = self.last_download
        self.last_time = time.time()

        # Canvas for custom shape / Özel şekil için tuval
        self.canvas = tk.Canvas(self.root, bg=self.transparent_key, highlightthickness=0)
        self.canvas.pack(fill="both", expand=True)
        
        # Draw Rounded Rectangle / Yuvarlak Dikdörtgen Çiz
        self.draw_rounded_rect()

        # UI Components on Canvas / Tuval üzerindeki arayüz bileşenleri
        self.setup_ui()
        
        # Drag Functionality / Sürükleme Özelliği
        self.root.bind("<Button-1>", self.start_move)
        self.root.bind("<B1-Motion>", self.do_move)
        self.canvas.bind("<Button-1>", self.start_move)
        self.canvas.bind("<B1-Motion>", self.do_move)
        
        # Start Updating / Güncellemeyi Başlat
        self.update_stats()

    def create_rounded_rect(self, x1, y1, x2, y2, radius=25, **kwargs):
        # Create a polygon for rounded rectangle / Yuvarlak dikdörtgen için çokgen oluştur
        points = [x1+radius, y1,
                  x1+radius, y1,
                  x2-radius, y1,
                  x2-radius, y1,
                  x2, y1,
                  x2, y1+radius,
                  x2, y1+radius,
                  x2, y2-radius,
                  x2, y2-radius,
                  x2, y2,
                  x2-radius, y2,
                  x2-radius, y2,
                  x1+radius, y2,
                  x1+radius, y2,
                  x1, y2,
                  x1, y2-radius,
                  x1, y2-radius,
                  x1, y1+radius,
                  x1, y1+radius,
                  x1, y1]
        return self.canvas.create_polygon(points, **kwargs, smooth=True)

    def draw_rounded_rect(self):
        # Draw background with border / Kenarlıklı arka plan çiz
        # Using a slight inset to ensure border is visible / Kenarlığın görünmesi için hafif iç boşluk kullanılıyor
        self.create_rounded_rect(2, 2, self.width-2, self.height-2, radius=20, 
                               fill=BG_COLOR, outline=ACCENT_COLOR, width=2)

    def setup_ui(self):
        # Helper to place widgets on canvas / Bileşenleri tuvale yerleştirmek için yardımcı
        # Note: We use place relative to root, but since canvas fills root, it works visually on top
        # Actually better to place them on the root or canvas. `place` works on widgets.
        
        # Container for content (to keep it organized) / İçerik için kapsayıcı
        # We'll use transparent frames or just direct placement.
        # Direct placement on canvas using create_window is cleaner for updates, 
        # but standard widgets with place() is easier for layout if background matches.
        # Problem: Standard Frame bg is rectangle. We need to rely on the main canvas bg.
        # So we should use Labels with BG_COLOR directly placed.
        
        # Close Button / Kapatma Butonu
        close_btn = tk.Label(self.root, text="✕", bg=BG_COLOR, fg=TEXT_MAIN, font=("Arial", 12, "bold"), cursor="hand2")
        close_btn.place(x=self.width-30, y=10)
        close_btn.bind("<Button-1>", lambda e: self.root.destroy())
        
        # Title / Başlık
        title_label = tk.Label(self.root, text="Network Monitor", bg=BG_COLOR, fg=ACCENT_COLOR, font=("Segoe UI", 10, "bold"))
        title_label.place(x=20, y=10)

        # Upload Speed Label / Yükleme Hızı Etiketi
        self.lbl_upload = tk.Label(self.root, text="Upload: 0 KB/s", bg=BG_COLOR, fg=TEXT_HIGHLIGHT, font=("Segoe UI", 11))
        self.lbl_upload.place(x=20, y=45)
        
        # Download Speed Label / İndirme Hızı Etiketi
        self.lbl_download = tk.Label(self.root, text="Download: 0 KB/s", bg=BG_COLOR, fg=TEXT_HIGHLIGHT, font=("Segoe UI", 11))
        self.lbl_download.place(x=20, y=70)

        # Divider (Line) / Ayırıcı Çizgi
        self.canvas.create_line(20, 105, self.width-20, 105, fill=ACCENT_COLOR)

        # Totals / Toplamlar
        self.lbl_total_upload = tk.Label(self.root, text="Total Up: 0 MB", bg=BG_COLOR, fg=TEXT_MAIN, font=("Segoe UI", 9))
        self.lbl_total_upload.place(x=20, y=115)
        
        self.lbl_total_download = tk.Label(self.root, text="Total Down: 0 MB", bg=BG_COLOR, fg=TEXT_MAIN, font=("Segoe UI", 9))
        self.lbl_total_download.place(x=20, y=135)

    def start_move(self, event):
        self.x = event.x
        self.y = event.y

    def do_move(self, event):
        deltax = event.x - self.x
        deltay = event.y - self.y
        x = self.root.winfo_x() + deltax
        y = self.root.winfo_y() + deltay
        self.root.geometry(f"+{x}+{y}")

    def format_bytes(self, size):
        power = 2**10
        n = 0
        power_labels = {0 : '', 1: 'KB', 2: 'MB', 3: 'GB', 4: 'TB'}
        while size > power:
            size /= power
            n += 1
        return f"{size:.2f} {power_labels[n]}"

    def update_stats(self):
        try:
            current_upload = psutil.net_io_counters().bytes_sent
            current_download = psutil.net_io_counters().bytes_recv
            current_time = time.time()
            
            dt = current_time - self.last_time
            if dt > 0:
                upload_speed = (current_upload - self.last_upload) / dt
                download_speed = (current_download - self.last_download) / dt
                
                total_upload_session = current_upload - self.start_upload_total
                total_download_session = current_download - self.start_download_total
                
                self.lbl_upload.config(text=f"Upload: {self.format_bytes(upload_speed)}/s")
                self.lbl_download.config(text=f"Download: {self.format_bytes(download_speed)}/s")
                
                self.lbl_total_upload.config(text=f"T. Up: {self.format_bytes(total_upload_session)}")
                self.lbl_total_download.config(text=f"T. Down: {self.format_bytes(total_download_session)}")
                
                self.last_upload = current_upload
                self.last_download = current_download
                self.last_time = current_time
        except Exception as e:
            print(f"Error updating stats: {e}")

        self.root.after(1000, self.update_stats)

if __name__ == "__main__":
    root = tk.Tk()
    app = NetworkMonitorApp(root)
    root.mainloop()
