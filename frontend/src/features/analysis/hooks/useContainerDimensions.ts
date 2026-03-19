import { useState, useEffect, type RefObject } from "react";

export function useContainerDimensions(
  ref: RefObject<HTMLDivElement | null>
): { width: number; height: number } {
  const [dimensions, setDimensions] = useState({ width: 0, height: 0 });

  useEffect(() => {
    const element = ref.current;
    if (!element) return;

    let timeoutId: ReturnType<typeof setTimeout> | null = null;

    const observer = new ResizeObserver((entries) => {
      if (timeoutId) clearTimeout(timeoutId);
      timeoutId = setTimeout(() => {
        const entry = entries[0];
        if (entry) {
          setDimensions({
            width: entry.contentRect.width,
            height: entry.contentRect.height,
          });
        }
      }, 150);
    });

    observer.observe(element);

    return () => {
      if (timeoutId) clearTimeout(timeoutId);
      observer.disconnect();
    };
  }, [ref]);

  return dimensions;
}
