/// <reference types="astro/client" />
/** Global App Context */
type Context = {
    /** Refresh Time in minutes */
    refreshTime: RefreshTime;
    /** Is server must fetch data */
    enabled: bool;
};
