/* This file is generated and managed by tsync */

/** Data needed to connect to a BOB cluster */
interface BobConnectionData {
  /** Address to connect to */
  hostname: Hostname;
  /** [Optional] Credentials used for BOB authentication */
  credentials?: Credentials;
}

/** Optional auth credentials for a BOB cluster */
interface Credentials {
  /** Login used during auth */
  login: string;
  /** Password used during auth */
  password: string;
}
