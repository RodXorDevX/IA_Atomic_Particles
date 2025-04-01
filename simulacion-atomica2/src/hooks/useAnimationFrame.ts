import { useEffect, useRef } from 'react';

export const useAnimationFrame = (callback: () => void, isActive: boolean = true) => {
    const requestRef = useRef<number>();
    const previousTimeRef = useRef<number>();

    useEffect(() => {
        if (!isActive) return;

        const animate = (time: number) => {
            if (previousTimeRef.current !== undefined) {
                callback();
            }
            previousTimeRef.current = time;
            requestRef.current = requestAnimationFrame(animate);
        };

        requestRef.current = requestAnimationFrame(animate);
        return () => {
            if (requestRef.current) {
                cancelAnimationFrame(requestRef.current);
            }
        };
    }, [callback, isActive]);
}; 