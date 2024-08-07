import axios, {
    AxiosInstance,
    AxiosError,
    AxiosRequestConfig,
    AxiosResponse,
} from 'axios'
import { ResultData } from './api_typing';
import { ResultEnum } from '../utils/constants'
// import { jumpSSOLogin } from '@/utils/login'

const config = {
    // 默认地址请求地址，可在 .env.*** 文件中修改
    baseURL: process.env.base_url,
    // 设置超时时间（30s）
    timeout: ResultEnum.TIMEOUT as number,
    // 跨域时候允许携带凭证
    // withCredentials: true,
}

export const jumpLogin = () => {
    // window.localStorage.setItem(
    //     'exitPathname',
    //     `${window.location.pathname}${window.location.search}`
    // )
    const redirectUrl = `${window.location.origin}/login`;
    console.log("redirect to ", redirectUrl);
    window.open(redirectUrl, '_self');
    // window.open(
    //     `https://sso.cowarobot.com?app_id=sso_a8b4f3c14181&app_url=${encodeURIComponent(
    //         redirectUrl
    //     )}#login`,
    //     '_self'
    // )
}

class RequestHttp {
    service: AxiosInstance
    public constructor(config: AxiosRequestConfig) {
        // 实例化axios
        this.service = axios.create(config)

        /**
         * @description 响应拦截器
         *  服务器换返回信息 -> [拦截统一处理] -> 客户端JS获取到信息
         */
        this.service.interceptors.response.use(
            (response: AxiosResponse) => {
                const { data, config } = response
                // ! 特殊获取地图相关的数据无code，success等字段，需要单独判断, 该场景下只要走到该回调就是成功
                if ((data?.code ?? null) === null) {
                    return data
                }

                // * 其他业务场景请求成功 存在data.code && data.code === 0 并且 data.success === true
                if (data.code === ResultEnum.SUCCESS && data.status) {
                    return data
                } else {
                    return data
                    console.error(`${data?.message}\n url: ${config?.url}`)
                    return Promise.reject(data)
                }
            },
            async (error: AxiosError | any) => {
                const { response } = error
                if (response.status === 401) {
                    console.error(401);
                    jumpLogin()
                } else if (response.status === 403) {
                    // window.open(
                    //     `/sanitation/forbidden?errorMsg=${response?.data?.message}`,
                    //     '_self'
                    // )
                    console.error(403);
                } else {
                    console.error(response.status);
                    return Promise.reject(error)
                }
            }
        )
    }

    // * 常用请求方法封装
    get<T>(url: string, params?: object, _object = {}): Promise<ResultData<T>> {
        return this.service.get(url, {
            params,
            ..._object,
        })
    }
    post<T>(url: string, params?: object, _object = {}): Promise<ResultData<T>> {
        return this.service.post(url, params, _object)
    }
    put<T>(url: string, params?: object, _object = {}): Promise<ResultData<T>> {
        return this.service.put(url, params, _object)
    }
    delete<T>(url: string, params?: any, _object = {}): Promise<ResultData<T>> {
        return this.service.delete(url, { params, ..._object })
    }
    download(url: string, params?: object, _object = {}): Promise<Blob> {
        return this.service.post(url, params, { ..._object, responseType: 'blob' })
    }
}

// eslint-disable-next-line import/no-anonymous-default-export
export default new RequestHttp(config)