export type TAtomClasses = string; // TODO

export declare type LinariaClassName = string & {
  __linariaClassName: true;
};

export declare type CSSProperties = {
  [key: string]: string | number | CSSProperties;
};

export declare type StyledMeta = {
  __linaria: {
    className: string;
    extends: StyledMeta;
  };
};

export declare type CSS = (
  strings: TemplateStringsArray,
  ...exprs: Array<string | number | CSSProperties | StyledMeta>
) => LinariaClassName;

export type CssClass =
  | TAtomClasses
  | LinariaClassName
  | false
  | null
  | undefined
  | ""; // if strictNullCheck is disabled

export type CxArgs = (CssClass | CssClass[])[];

export type TAtomClassesOrArray = CssClass | CssClass[];

export type ITokens = any;
