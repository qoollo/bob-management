import { persistentMap } from '@nanostores/persistent';

const context: Context = {
    refreshTime: '1',
    enabled: false,
};
export const Context = persistentMap<Context>('context', context);
