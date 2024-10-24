import { createApp } from "vue";
import App from "./App.vue";
import ElementPlus from 'element-plus';
import 'element-plus/dist/index.css';
import router from './router';
import { createI18n } from 'vue-i18n';

const messages = {
  en: {
    dictionary: 'Dictionary',
    file: 'File',
    textReplacement: 'Text Replacement',
    replacementDictionary: 'Replacement Dictionary',
    selectDictionaryFile: 'Select Dictionary File',
    batchAdd: 'Batch Add',
    find: 'Find',
    replaceWith: 'Replace With',
    action: 'Action',
    delete: 'Delete',
    add: 'Add',
    generatedEmojis: 'Generated emojis',
    submit: 'Submit',
    cancel: 'Cancel',
    replacementAddedSuccessfully: 'Replacement added successfully',
    enterTextHere: 'Enter text here...',
    replace: 'Replace',
    reverseReplace: 'Reverse Replace',
    convertToUnicode: 'Convert to Unicode',
    unicodeToTextAndReverseReplace: 'Unicode to Text & Reverse Replace',
    copyResultToClipboard: 'Copy Result to Clipboard',
    replacedTextWillAppearHere: 'Replaced text will appear here...',
    textCopiedToClipboard: 'Text copied to clipboard!',
    failedToCopyText: 'Failed to copy text: ',
    fileProcessing: 'File Processing',
    selectFile: 'Select File',
    executeReplaceAndSave: 'Execute Replace and Save',
    reverseReplaceAndSave: 'Reverse Replace and Save',
    selectedFile: 'Selected File',
    fileSavedAs: 'File saved as: {path}',
    errorProcessingFile: 'Error processing file'
  },
  zh: {
    dictionary: '词典',
    file: '文件',
    textReplacement: '文本替换',
    replacementDictionary: '替换字典',
    selectDictionaryFile: '选择字典文件',
    batchAdd: '批量添加',
    find: '查找',
    replaceWith: '替换为',
    action: '操作',
    delete: '删除',
    add: '添加',
    generatedEmojis: '生成的表情符号',
    submit: '提交',
    cancel: '取消',
    replacementAddedSuccessfully: '替换项添加成功',
    enterTextHere: '在此输入文本...',
    replace: '替换',
    reverseReplace: '反向替换',
    convertToUnicode: '转换为Unicode',
    unicodeToTextAndReverseReplace: 'Unicode转文本并反向替换',
    copyResultToClipboard: '复制结果到剪贴板',
    replacedTextWillAppearHere: '替换后的文本将显示在这里...',
    textCopiedToClipboard: '文本已复制到剪贴板！',
    failedToCopyText: '复制文本失败：',
    fileProcessing: '文件处理',
    selectFile: '选择文件',
    executeReplaceAndSave: '执行替换并保存',
    reverseReplaceAndSave: '反向替换并保存',
    selectedFile: '已选择文件',
    fileSavedAs: '文件已保存为：{path}',
    errorProcessingFile: '处理文件时出错'
  }
};

const i18n = createI18n({
  legacy: false, // 使用 Composition API 模式
  locale: 'zh', // 设置默认语言
  fallbackLocale: 'en', // 设置回退语言
  messages,
});

const app = createApp(App);
app.use(ElementPlus);
app.use(router);
app.use(i18n);
app.mount('#app');
