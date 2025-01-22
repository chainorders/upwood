import { Outlet } from "react-router";

export default function UnAuthLayout() {
    return (
        <div>
            <h1>Un Auth Layout</h1>
            <Outlet />
        </div>
    )
}