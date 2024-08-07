export interface Result {
    code: number;
    status: boolean;
}

export interface ResultData<T = any> extends Result {
    message: T
}

export interface ResultDataList<T = any> extends Result {
    message: T[]
}

// * 分页请求参数
export interface ReqPage {
    page_number: number
    page_size: number
}

export interface ResPage<T> extends Extra {
    list: T[]
    page_number?: number
    page_size?: number
    total_items: number
    total_pages?: number
}