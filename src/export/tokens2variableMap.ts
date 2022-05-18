import type { ITokens } from '../types';
import { capitalize } from './capitalize';

interface ITokens2VarMapOptions {
  onPropNotFound?: (info: { category: string }) => void;
}

export const propShortcuts = {
  color: ['', ''],
  font: ['', ''],
  gradient: ['g', ''],
  spacing: ['s', 'px'],
  letterSpacing: ['ls', 'px'],
  paragraphSpacing: ['ps', 'px'],
  borderRadius: ['br', 'px'],
  opacity: ['o', '%'],
  lineHeight: ['lh', 'em'],
  fontSize: ['fs', 'em'],
  headlines: null, // ignore
};

export const disablePropCapitalize = ['color', 'font'];

export function tokens2variableMap(
  tokens: ITokens,
  { onPropNotFound }: ITokens2VarMapOptions = {}
): Record<string, string> {
  const variables = {};

  for (const category in tokens) {
    for (const prop in tokens[category]) {
      const value = tokens[category][prop];
      const propName = disablePropCapitalize.includes(category)
        ? prop
        : capitalize(prop);
      const propConfig = propShortcuts[category];

      if (propConfig === null) {
        continue;
      } else if (propConfig === undefined) {
        onPropNotFound && onPropNotFound({ category });
        continue;
      }

      const [short, unit] = propConfig;
      variables[`${short}${propName}`] = `${value}${unit}`;
    }
  }

  return variables;
}
