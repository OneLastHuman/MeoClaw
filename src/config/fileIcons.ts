// 文件类型 → 图标文件名的映射
export type FileIconType = 'code' | 'doc' | 'docs' | 'music' | 'pic' | 'text' | 'url' | 'video';

export const FILE_ICON_MAP: Record<string, FileIconType> = {
  // 代码/开发
  js: 'code', ts: 'code', py: 'code', java: 'code', c: 'code', cpp: 'code',
  h: 'code', css: 'code', html: 'code', vue: 'code', tsx: 'code', jsx: 'code',
  go: 'code', rs: 'code', rb: 'code', php: 'code', swift: 'code', kt: 'code',
  sh: 'code', bash: 'code', zsh: 'code', json: 'code', xml: 'code', yaml: 'code',
  yml: 'code', toml: 'code', md: 'code', sql: 'code', apk: 'code', ipa: 'code',
  // 单文档
  pdf: 'doc', doc: 'doc', docx: 'doc', xls: 'doc', xlsx: 'doc', ppt: 'doc',
  pptx: 'doc', odt: 'doc', ods: 'doc', odp: 'doc',
  // 文件夹/压缩包
  zip: 'docs', rar: 'docs', '7z': 'docs', tar: 'docs', gz: 'docs', dmg: 'docs',
  pkg: 'docs',
  // 音频
  mp3: 'music', wav: 'music', flac: 'music', aac: 'music', ogg: 'music',
  m4a: 'music', wma: 'music', ape: 'music',
  // 图片
  jpg: 'pic', jpeg: 'pic', png: 'pic', gif: 'pic', bmp: 'pic', svg: 'pic',
  webp: 'pic', ico: 'pic', tiff: 'pic', psd: 'pic', raw: 'pic',
  // 纯文本
  txt: 'text', log: 'text', csv: 'text',
  // 链接
  url: 'url', webloc: 'url', desktop: 'url',
  // 视频
  mp4: 'video', avi: 'video', mkv: 'video', mov: 'video', wmv: 'video',
  flv: 'video', webm: 'video', m4v: 'video', mpg: 'video', mpeg: 'video',
};

export function getFileIcon(filename: string): FileIconType {
  const ext = filename.split('.').pop()?.toLowerCase() ?? '';
  return FILE_ICON_MAP[ext] ?? 'docs';
}
