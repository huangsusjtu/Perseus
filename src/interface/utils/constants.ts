export const TOKEN = "TOKEN"

export const ADMIN = "admin"

export const LOGIN_URL = "/login"

export const ROUTER_WHITE_LIST: string[] = ["/login"]

export const BAIDU_MAP_KEY: string = "aoxQr3uFO8ciYN0Fcgta821p0lbD9Ib6"

export enum RequestEnum {
    GET = "GET",
    POST = "POST",
    PATCH = "PATCH",
    PUT = "PUT",
    DELETE = "DELETE",
}

export enum ResultEnum {
    SUCCESS = 200,
    ERROR = 500,
    OVERDUE = 401,
    NOTACCEPT = 406,
    TIMEOUT = 100000,
    BADREQ = 400,
    TYPE = "success",
}

export enum ContentTypeEnum {
    JSON = "application/json;charset=UTF-8",
    TEXT = "text/plain;charset=UTF-8",
    FORM_URLENCODED = "application/x-www-form-urlencoded;charset=UTF-8", // form-data 一般配合qs
    FORM_DATA = "multipart/form-data;charset=UTF-8", // form-data 上传
}

export const checkStatus = (status: number): void => {
    switch (status) {
        case 400:
            console.error({ content: "请求失败！请您稍后重试" })
            break
        case 401:
            console.error({ content: "登录失效！请您重新登录" })
            break
        case 403:
            console.error({ content: "当前账号无权限访问！" })
            break
        case 404:
            console.error({ content: "你所访问的资源不存在！" })
            break
        case 405:
            console.error({ content: "请求方式错误！请您稍后重试" })
            break
        case 406:
            console.error({ content: "请重新登录" })
            break
        case 408:
            console.error({ content: "请求超时！请您稍后重试" })
            break
        case 500:
            console.error({ content: "服务异常！" })
            break
        case 502:
            console.error({ content: "网关错误！" })
            break
        case 503:
            console.error({ content: "服务不可用！" })
            break
        case 504:
            console.error({ content: "网关超时！" })
            break
        default:
            console.error({ content: "请求失败！" })
    }
}