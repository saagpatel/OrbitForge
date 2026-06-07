import { Component, type ErrorInfo, type ReactNode } from "react";

interface Props {
  children: ReactNode;
}

interface State {
  error: Error | null;
}

export class AppErrorBoundary extends Component<Props, State> {
  state: State = { error: null };

  static getDerivedStateFromError(error: Error): State {
    return { error };
  }

  componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error("App render failed:", error, errorInfo);
  }

  render() {
    if (!this.state.error) {
      return this.props.children;
    }

    return (
      <div className="h-full w-full bg-black text-white flex items-center justify-center p-6">
        <div className="max-w-lg rounded-lg border border-red-500/30 bg-red-950/40 p-4 text-sm">
          <div className="font-semibold text-red-200">
            OrbitForge hit a UI error
          </div>
          <div className="mt-2 text-red-100/80">{this.state.error.message}</div>
        </div>
      </div>
    );
  }
}
