import { theme, type CustomFlowbiteTheme } from "flowbite-react";
import { twMerge } from "tailwind-merge";

export const customTheme: CustomFlowbiteTheme = {
  checkbox: {
    root: {
      color: {
        default:
          "border-gray-300 bg-gray-50 focus:ring-3 focus:ring-primary-300 dark:focus:ring-primary-600 dark:ring-offset-gray-800 dark:bg-gray-700 dark:border-gray-600",
      },
    },
  },
  modal: {
    content: {
      inner: twMerge(theme.modal.content.inner, "dark:bg-gray-800"),
    },
    header: {
      base: twMerge(
        theme.modal.header.base,
        "items-center dark:border-gray-700",
      ),
      title: twMerge(theme.modal.header.title, "font-semibold"),
      close: {
        base: twMerge(
          theme.modal.header.close.base,
          "hover:bg-gray-200 dark:hover:bg-gray-700",
        ),
      },
    },
    footer: {
      base: twMerge(theme.modal.footer.base, "dark:border-gray-700"),
    },
  },
  progress: {
    color: {
      blue: "bg-primary-600",
      dark: "bg-gray-900 dark:bg-white",
    },
    size: {
      md: "h-2",
    },
  },
  select: {
    field: {
      select: {
        sizes: {
          md: twMerge(
            theme.select.field.select.sizes.md,
            "text-base sm:text-sm",
          ),
        },
        colors: {
          gray: twMerge(
            theme.select.field.select.colors.gray,
            "focus:border-blue-500 focus:ring-blue-500 dark:focus:border-blue-500 dark:focus:ring-blue-500",
          ),
        },
      },
    },
  },
  sidebar: {
    root: {
      inner: twMerge(theme.sidebar.root.inner, "bg-white"),
    },
    collapse: {
      button: twMerge(
        theme.sidebar.collapse.button,
        "text-gray-900 hover:bg-gray-100 dark:text-gray-200 dark:hover:bg-gray-700 text-xl",
      ),
    },
    item: {
      // 通过p-3可以控制各个item之间的间距
      base: twMerge(
        theme.sidebar.collapse.button,
        "text-gray-900 hover:bg-gray-100 dark:text-gray-200 dark:hover:bg-gray-700 text-base py-3",
      ),
      label:
        "inline-flex justify-center items-center p-1 ml-3 w-5 h-5 text-sm font-medium rounded-full text-primary-800 bg-primary-100",
      // 通过icon可以控制sidebar的图标风格，text-xl可以控制图标的大小
      icon: {
        base: "text-gray-900 dark:text-gray-100 hover:text-gray-900 dark:hover:text-white text-xl",
      },
    },
    itemGroup: {
      base: "list-none border-t border-gray-200 pt-3 first:mt-0 first:border-t-0 first:pt-0 dark:border-gray-700",
    },

  },
  textarea: {
    base: twMerge(theme.textarea.base, "p-4"),
    colors: {
      gray: twMerge(
        theme.textarea.colors.gray,
        "text-base focus:border-blue-500 focus:ring-blue-500 sm:text-sm dark:focus:border-blue-500 dark:focus:ring-blue-500",
      ),
    },
  },
  textInput: {
    field: {
      input: {
        base: twMerge(theme.textInput.field.input.base, "outline-none"),
        sizes: {
          md: "sm:text-sm p-2.5",
        },
        colors: {
          gray: twMerge(
            theme.textInput.field.input.colors.gray,
            "focus:border-primary-500 focus:ring-primary-500 dark:focus:border-primary-500 dark:focus:ring-primary-500",
          ),
        },
      },
    },
  },
  toggleSwitch: {
    toggle: {
      base: "toggle-bg rounded-full border",
      checked: {
        color: {
          blue: "bg-blue-600 border-blue-600",
        },
      },
    },
  },
  card: {
    root: {
      base: twMerge(theme.card.root.base, "border-none shadow"),
      children: "p-4 sm:p-6 xl:p-8",
    },
  },
};
