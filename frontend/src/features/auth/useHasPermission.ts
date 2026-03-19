import { useAuth } from "./useAuth";

const PERMISSIONS: Record<string, Record<string, Record<string, boolean>>> = {
  risk_manager: {
    ontology: { read: true, create: true, update: true },
    compliance: { read: true, create: true, update: true, delete: true },
    analysis: { read: true, create: true, update: true, delete: true },
    reports: { read: true, export: true },
    auth: {},
  },
  specialist: {
    ontology: { read: true },
    compliance: { read: true, create: true, update: true },
    analysis: { read: true, create: true, update: true },
    reports: { read: true, export: true },
    auth: {},
  },
  viewer: {
    ontology: { read: true },
    compliance: { read: true },
    analysis: { read: true },
    reports: { read: true },
    auth: {},
  },
};

export function useHasPermission(feature: string, action: string): boolean {
  const { user } = useAuth();

  if (!user) return false;

  const role = user.role.toLowerCase();

  if (role === "admin") return true;

  return PERMISSIONS[role]?.[feature]?.[action] ?? false;
}
