import { persistentMap } from '@nanostores/persistent';

export type RefreshTime = (typeof refreshTimes)[number];
export const refreshTimes = ['1', '5', '15', '30'];

const context: Context = {
    refreshTime: '1',
    enabled: false,
};
export const Context = persistentMap<Context>('context', context);
