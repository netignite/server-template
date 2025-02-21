/* Generated File -- DO NOT TOUCH */
/* eslint-disable */
export type Response<T> = { data: T }

export type ErrorResponse<T> = { message: string; code: number | null; data: T | null }

export type ResponseType<T> = ({ type: "success" } & Response<T>) | ({ type: "fail" } & Response<T>) | ({ type: "error" } & ErrorResponse<T>)
