type TauriEvent<T> = {
    payload: T
};

type RustResult<T> = {
    Ok: T | null,
    Err: string | null,
}

export type { TauriEvent, RustResult };
