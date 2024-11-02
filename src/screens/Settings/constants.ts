import { tags as t } from '@lezer/highlight'
import createTheme from '@uiw/codemirror-themes'

import type { BasicSetupOptions } from '@uiw/react-codemirror'
import type { Core } from '../../core/types'

export const CODE_MIRROR_OPTIONS: BasicSetupOptions = {
  allowMultipleSelections: false,
  autocompletion: false,
  bracketMatching: false,
  closeBrackets: false,
  closeBracketsKeymap: false,
  completionKeymap: false,
  crosshairCursor: false,
  defaultKeymap: false,
  drawSelection: false,
  dropCursor: false,
  foldGutter: false,
  foldKeymap: false,
  highlightActiveLine: true,
  highlightActiveLineGutter: false,
  highlightSelectionMatches: false,
  highlightSpecialChars: false,
  history: true,
  historyKeymap: true,
  indentOnInput: false,
  lineNumbers: true,
  lintKeymap: false,
  rectangularSelection: false,
  searchKeymap: false,
  syntaxHighlighting: true,
}

export const CODE_MIRROR_THEME = createTheme({
  settings: {
    background: '#000000',
    caret: '#c9d1d9',
    fontFamily: "'Reddit Mono', monospace",
    fontSize: '90%',
    foreground: '#c9d1d9',
    gutterBackground: 'transparent',
    gutterBorder: 'transparent',
    gutterForeground: '#787c99',
    lineHighlight: '#36334280',
    selection: '#003d73',
    selectionMatch: '#003d73',
  },
  styles: [
    { color: '#bb9af7', tag: t.keyword },
    { color: '#c0caf5', tag: [t.name, t.deleted, t.character, t.macroName] },
    { color: '#7aa2f7', tag: [t.propertyName] },
    { color: '#9ece6a', tag: [t.processingInstruction, t.string, t.inserted, t.special(t.string)] },
    { color: '#7aa2f7', tag: [t.function(t.variableName), t.labelName] },
    { color: '#bb9af7', tag: [t.color, t.constant(t.name), t.standard(t.name)] },
    { color: '#c0caf5', tag: [t.definition(t.name), t.separator] },
    { color: '#c0caf5', tag: [t.className] },
    { color: '#ff9e64', tag: [t.number, t.changed, t.annotation, t.modifier, t.self, t.namespace] },
    { color: '#0db9d7', tag: [t.typeName] },
    { color: '#bb9af7', tag: [t.operator, t.operatorKeyword] },
    { color: '#b4f9f8', tag: [t.url, t.escape, t.regexp, t.link] },
    { color: '#444b6a', tag: [t.meta, t.comment] },
    { fontWeight: '600', tag: t.strong },
    { fontStyle: 'italic', tag: t.emphasis },
    { fontWeight: '400', tag: t.link, textDecoration: 'underline' },
    { color: '#89ddff', fontWeight: '600', tag: t.heading },
    { color: '#c0caf5', tag: [t.atom, t.bool, t.special(t.variableName)] },
    { color: '#ff5370', tag: t.invalid },
    { fontWeight: '400', tag: t.strikethrough, textDecoration: 'line-through' },
  ],
  theme: 'dark',
})

export const INITIAL_SETTINGS_STATE: Core.SettingsState = {
  // biome-ignore lint/style/useNamingConvention: <explanation>
  clamd_conf_file_path: null,
  // biome-ignore lint/style/useNamingConvention: <explanation>
  clamd_conf_file_source: null,
  // biome-ignore lint/style/useNamingConvention: <explanation>
  is_ready: false,
  // biome-ignore lint/style/useNamingConvention: <explanation>
  is_writing: false,
}
