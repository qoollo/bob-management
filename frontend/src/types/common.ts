export const cookieAuthId: string = 'id';
type Valuable<T> = { [K in keyof T as T[K] extends null | undefined ? never : K]: T[K] };

export function removeEmpty<
    // eslint-disable-next-line @typescript-eslint/ban-types
    T extends {},
    V = Valuable<T>,
>(obj: T): V {
    return Object.fromEntries(
        Object.entries(obj).filter(
            ([, v]) =>
                !(
                    (typeof v === 'string' && v === '') ||
                    v === null ||
                    typeof v === 'undefined' ||
                    (typeof v === 'object' && Object.keys(removeEmpty(v)).length === 0)
                ),
        ),
    ) as V;
}

export function getCookie(field: string) {
    const entry = document.cookie.split(';').find((e) => e.replace(' ', '').startsWith(field + '='));
    if (entry) {
        return entry.split('=')[1];
    } else {
        return '';
    }
}

export function eraseCookie(name: string) {
    document.cookie = name + '=; Max-Age=-99999999;';
}
