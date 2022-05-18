import cx from './classnames';
export type CxArgsUntyped = (string | string[])[];
export default cx as (...args: CxArgsUntyped) => string;
