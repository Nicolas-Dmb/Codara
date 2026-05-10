import { useState } from "react";

export default function useColumn() {
    const [isOpen, setIsOpen] = useState(true);

    const toggleColumn = () => {
        setIsOpen(!isOpen);
    };

    return { isOpen, toggleColumn };
}
