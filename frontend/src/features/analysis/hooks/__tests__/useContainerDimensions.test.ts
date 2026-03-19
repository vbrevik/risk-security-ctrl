import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { renderHook, act } from "@testing-library/react";
import { useContainerDimensions } from "../useContainerDimensions";

let mockCallback: ResizeObserverCallback;
const mockObserve = vi.fn();
const mockDisconnect = vi.fn();

class MockResizeObserver {
  constructor(callback: ResizeObserverCallback) {
    mockCallback = callback;
  }
  observe = mockObserve;
  unobserve = vi.fn();
  disconnect = mockDisconnect;
}

beforeEach(() => {
  vi.useFakeTimers();
  vi.stubGlobal("ResizeObserver", MockResizeObserver);
  mockObserve.mockClear();
  mockDisconnect.mockClear();
});

afterEach(() => {
  vi.useRealTimers();
  vi.unstubAllGlobals();
});

function fireResize(width: number, height: number) {
  mockCallback(
    [{ contentRect: { width, height } } as ResizeObserverEntry],
    {} as ResizeObserver
  );
}

describe("useContainerDimensions", () => {
  it("returns initial dimensions of { width: 0, height: 0 }", () => {
    const ref = { current: document.createElement("div") };
    const { result } = renderHook(() => useContainerDimensions(ref));
    expect(result.current).toEqual({ width: 0, height: 0 });
  });

  it("updates dimensions when ResizeObserver fires", () => {
    const ref = { current: document.createElement("div") };
    const { result } = renderHook(() => useContainerDimensions(ref));

    act(() => {
      fireResize(600, 400);
      vi.advanceTimersByTime(150);
    });

    expect(result.current).toEqual({ width: 600, height: 400 });
  });

  it("cleans up observer on unmount", () => {
    const ref = { current: document.createElement("div") };
    const { unmount } = renderHook(() => useContainerDimensions(ref));
    unmount();
    expect(mockDisconnect).toHaveBeenCalledTimes(1);
  });

  it("debounces rapid resize events", () => {
    const ref = { current: document.createElement("div") };
    const { result } = renderHook(() => useContainerDimensions(ref));

    act(() => {
      fireResize(100, 100);
      vi.advanceTimersByTime(50);
      fireResize(200, 200);
      vi.advanceTimersByTime(50);
      fireResize(300, 300);
      vi.advanceTimersByTime(150);
    });

    expect(result.current).toEqual({ width: 300, height: 300 });
  });
});
