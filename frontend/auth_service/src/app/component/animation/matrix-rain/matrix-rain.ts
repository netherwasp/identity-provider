import { Component, ElementRef, viewChild, HostListener } from '@angular/core';

@Component({
  selector: 'app-matrix-rain',
  imports: [],
  templateUrl: './matrix-rain.html',
  styleUrl: './matrix-rain.scss',
})
export class MatrixRain {
  canvasRef = viewChild.required<ElementRef<HTMLCanvasElement>>('matrain');
  private ctx!: CanvasRenderingContext2D;
  private rafId = 0;
  columns = 0; // column size per rain
  speeds: number[] = []; // fall speed per column
  drops: number[] = []; // current vertical position

  FONT_SIZE = 12; // size of character in px
  HEAD = '#70ff9e';
  TAIL = '#00ff41';
  V_SPACING = 240; // vertical spacing for head and tail
  char_set: string[] = Array.from({ length: 94 }, (_, i) => String.fromCharCode(33 + i));

  SPEED_MULTIPLIER = 0.8; // try 0.05–0.15, lower = slower
  UPDATE_INTERVAL_MS = 80; // ms between updates; higher = calmer, more readable
  private lastUpdate = 0;

  GLYPH_INTERVAL_MS = 250; // how often symbols change, independent of fall speed
  currentGlyphs: string[] = [];
  private lastGlyphUpdate = 0;

  glyph = () => this.char_set[(Math.random() * this.char_set.length) | 0];

  get canvas() {
    return this.canvasRef().nativeElement;
  }

  async ngAfterViewInit() {
    this.ctx = this.canvas.getContext('2d')!;

    try {
      await document.fonts.load(`${this.FONT_SIZE}px "OCR-A"`);
      await document.fonts.ready; // wait font for to load
    } catch (err) {
      console.warn('OCR-A font failed to load, falling back to monospace', err);
    }

    this.resize();
    this.rafId = requestAnimationFrame(this.loop);
  }

  ngOnDestroy() {
    cancelAnimationFrame(this.rafId);
  }
  @HostListener('window:resize')
  resize() {
    this.canvas.width = window.innerWidth;
    this.canvas.height = window.innerHeight;

    this.columns = Math.floor(this.canvas.width / this.FONT_SIZE);
    this.drops = Array.from({ length: this.columns }, () => (Math.random() * -50) | 0);
    this.speeds = Array.from(
      { length: this.columns },
      () => (0.3 + Math.random() * 0.8) * this.SPEED_MULTIPLIER,
    );

    this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
  }

  frame(timestamp: number) {
    const shouldChangeGlyph = timestamp - this.lastGlyphUpdate >= this.GLYPH_INTERVAL_MS;
    if (shouldChangeGlyph) this.lastGlyphUpdate = timestamp;

    this.ctx.globalCompositeOperation = 'destination-out';
    this.ctx.fillStyle = 'rgba(0,0,0,0.08)'; // color doesn't matter here, only alpha
    this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);

    this.ctx.globalCompositeOperation = 'source-over';
    this.ctx.font = this.FONT_SIZE + 'px "OCR-A", monospace';

    for (let i = 0; i < this.columns; i++) {
      const x = i * this.FONT_SIZE;
      const row = Math.floor(this.drops[i]);
      const y = row * this.FONT_SIZE;

      if (shouldChangeGlyph) {
        this.currentGlyphs[i] = this.glyph();
      }

      this.ctx.fillStyle = this.TAIL;
      this.ctx.fillText(this.glyph(), x, y + this.V_SPACING);
      this.ctx.fillStyle = this.HEAD;
      this.ctx.fillText(this.glyph(), x, y + this.FONT_SIZE);

      if (y > this.canvas.height && Math.random() > 0.975) this.drops[i] = 0;
      this.drops[i] += this.speeds[i];
    }
  }

  private loop = (timestamp: number) => {
    if (timestamp - this.lastUpdate >= this.UPDATE_INTERVAL_MS) {
      this.frame(timestamp);
      this.lastUpdate = timestamp;
    }
    this.rafId = requestAnimationFrame(this.loop);
  };
}
