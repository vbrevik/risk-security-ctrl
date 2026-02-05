import type { Concept, TreeNode } from "../types";

export function buildTree(concepts: Concept[], language: "en" | "nb" = "en"): TreeNode[] {
  const nodeMap = new Map<string, TreeNode>();
  const roots: TreeNode[] = [];

  // Create nodes for all concepts
  for (const concept of concepts) {
    const name = language === "nb" && concept.name_nb ? concept.name_nb : concept.name_en;
    nodeMap.set(concept.id, {
      id: concept.id,
      name,
      type: concept.concept_type,
      code: concept.code,
      children: [],
      concept,
      isExpanded: false,
    });
  }

  // Build tree structure
  for (const concept of concepts) {
    const node = nodeMap.get(concept.id)!;
    if (concept.parent_id && nodeMap.has(concept.parent_id)) {
      const parent = nodeMap.get(concept.parent_id)!;
      parent.children.push(node);
    } else {
      roots.push(node);
    }
  }

  // Sort children by sort_order
  const sortNodes = (nodes: TreeNode[]): TreeNode[] => {
    return nodes
      .sort((a, b) => {
        const orderA = a.concept.sort_order ?? 999;
        const orderB = b.concept.sort_order ?? 999;
        return orderA - orderB;
      })
      .map((node) => ({
        ...node,
        children: sortNodes(node.children),
      }));
  };

  return sortNodes(roots);
}

export function filterTree(nodes: TreeNode[], query: string): TreeNode[] {
  if (!query.trim()) return nodes;

  const lowerQuery = query.toLowerCase();

  const filterNode = (node: TreeNode): TreeNode | null => {
    const matchesName = node.name.toLowerCase().includes(lowerQuery);
    const matchesCode = node.code?.toLowerCase().includes(lowerQuery);
    const filteredChildren = node.children
      .map(filterNode)
      .filter((n): n is TreeNode => n !== null);

    if (matchesName || matchesCode || filteredChildren.length > 0) {
      return {
        ...node,
        children: filteredChildren,
        isExpanded: filteredChildren.length > 0,
      };
    }
    return null;
  };

  return nodes.map(filterNode).filter((n): n is TreeNode => n !== null);
}

export function findNodePath(nodes: TreeNode[], targetId: string): TreeNode[] {
  const path: TreeNode[] = [];

  const find = (nodeList: TreeNode[]): boolean => {
    for (const node of nodeList) {
      if (node.id === targetId) {
        path.push(node);
        return true;
      }
      if (find(node.children)) {
        path.unshift(node);
        return true;
      }
    }
    return false;
  };

  find(nodes);
  return path;
}
