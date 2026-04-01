// Preserves scroll position across .tex-triggered dev reloads.
//
// Algorithm components render asynchronously (katex, pseudocode, shiki),
// so after reload the restore function polls until the document is tall
// enough to scroll to the saved offset.

const STORAGE_KEY = "tex-reload-scroll";
const HMR_EVENT = "algo-reload";
const MAX_POLL_FRAMES = 120; // ~2 s at 60 fps
const TOLERANCE_PX = 1;

// Listen for .tex HMR events, save scroll offset, then reload.
export function saveScrollOnTexChange() {
  if (typeof window === "undefined" || !import.meta.hot) return;
  import.meta.hot.on(HMR_EVENT, () => {
    sessionStorage.setItem(STORAGE_KEY, String(window.scrollY));
    location.reload();
  });
}

// If a saved offset exists, polls until the page is tall enough to scroll.
export function restoreScroll() {
  if (typeof window === "undefined") return;
  const saved = sessionStorage.getItem(STORAGE_KEY);
  if (saved === null) return;

  sessionStorage.removeItem(STORAGE_KEY);
  const y = Number(saved);
  let frame = 0;

  const poll = () => {
    window.scrollTo(0, y);
    if (++frame < MAX_POLL_FRAMES && Math.abs(window.scrollY - y) > TOLERANCE_PX) {
      requestAnimationFrame(poll);
    }
  };
  requestAnimationFrame(poll);
}
