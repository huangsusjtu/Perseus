module.exports = {
  env: {
    browser: true,
    es2021: true,
  },
  overrides: [],
  parserOptions: {
    project: './tsconfig.json',
    ecmaVersion: 'latest',
    sourceType: 'module',
    ecmaFeatures: {
      // 表示你想使用的额外的语言特性
      jsx: true, // 启用 JSX
    },
  },
  parser: '@typescript-eslint/parser',
  extends: [
    'plugin:@typescript-eslint/recommended', // 使用 @typescript-eslint 插件推荐的规则
    'react-app',
    'plugin:prettier/recommended', // 使用 react 插件推荐的规则
  ],
  plugins: ['react-hooks'],
  rules: {
    // eslint (http://eslint.cn/docs/rules)
    'no-var': 'error', // 要求使用 let 或 const 而不是 var
    eqeqeq: [2, 'allow-null'], // 使用 === 替代 ==
    'no-multiple-empty-lines': ['error', { max: 1 }], // 不允许多个空行
    'no-multi-spaces': 2, // 不能用多余的空格
    'no-trailing-spaces': 2, // 一行结束后面不要有空格
    'no-use-before-define': 'off', // 禁止在 函数/类/变量 定义之前使用它们
    'prefer-const': 'off', // 此规则旨在标记使用 let 关键字声明但在初始分配后从未重新分配的变量，要求使用 const
    'no-irregular-whitespace': 'off', // 禁止不规则的空白
    semi: ['error', 'never'],
    // typeScript (https://typescript-eslint.io/rules)
    '@typescript-eslint/no-unused-vars': 'error', // 禁止定义未使用的变量
    '@typescript-eslint/prefer-ts-expect-error': 'error', // 禁止使用 @ts-ignore
    '@typescript-eslint/no-inferrable-types': 'off', // 可以轻松推断的显式类型可能会增加不必要的冗长
    '@typescript-eslint/no-namespace': 'off', // 禁止使用自定义 TypeScript 模块和命名空间。
    '@typescript-eslint/no-explicit-any': 'off', // 禁止使用 any 类型
    '@typescript-eslint/ban-types': 'off', // 禁止使用特定类型
    '@typescript-eslint/explicit-function-return-type': 'off', // 不允许对初始化为数字、字符串或布尔值的变量或参数进行显式类型声明
    '@typescript-eslint/no-var-requires': 'off', // 不允许在 import 语句中使用 require 语句
    '@typescript-eslint/no-empty-function': 'off', // 禁止空函数
    '@typescript-eslint/no-use-before-define': 'off', // 禁止在变量定义之前使用它们
    '@typescript-eslint/ban-ts-comment': 'off', // 禁止 @ts-<directive> 使用注释或要求在指令后进行描述
    '@typescript-eslint/no-non-null-assertion': 'off', // 不允许使用后缀运算符的非空断言(!)
    '@typescript-eslint/explicit-module-boundary-types': 'off', // 要求导出函数和类的公共类方法的显式返回和参数类型
    "react-hooks/rules-of-hooks": 'error',
    "react-hooks/exhaustive-deps": 'warn' // <--- THIS IS THE NEW RULE
  },
}
