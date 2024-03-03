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

export function formatBytes(bytes: number, decimals = 0) {
    if (!+bytes) return '0B';

    const k = 1024;
    const dm = decimals < 0 ? 0 : decimals;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];

    const i = Math.floor(Math.log(bytes) / Math.log(k));

    return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))}${sizes[i]}`;
}

export function proxiedPropertiesOf<TObj>() {
    return new Proxy(
        {},
        {
            get: (_, prop) => prop,
            set: () => {
                throw Error('Set not supported');
            },
        },
    ) as {
        [P in keyof TObj]?: P;
    };
}
