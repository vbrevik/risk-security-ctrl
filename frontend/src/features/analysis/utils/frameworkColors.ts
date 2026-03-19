import * as d3 from "d3";

export function buildFrameworkColorMap(
  frameworkIds: string[]
): (frameworkId: string) => string {
  const sorted = [...frameworkIds].sort();
  return (frameworkId: string) => {
    const index = sorted.indexOf(frameworkId);
    return d3.schemeTableau10[(index < 0 ? 0 : index) % 10];
  };
}

export function getFrameworkColor(
  frameworkIds: string[],
  frameworkId: string
): string {
  return buildFrameworkColorMap(frameworkIds)(frameworkId);
}
