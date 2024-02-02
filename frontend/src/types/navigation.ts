/// Const-asserted list of possible location on NavBar
export const locations = <const>['/dashboard', '/nodelist', '/vdisklist'];
/// Type defenition of locattions
export type NavLocation = (typeof locations)[number];
/// Type guard
export function isLocation(str: string): str is NavLocation {
    return !!locations.find((loc) => str === loc);
}
