'use client'


const title_bar_names =
    [
        'File',
        'Edit',
        'Selection',
        'View',
        'Go',
        'Run',
        'Terminal',
    ];

export default function TitleBar() {
    return (
        <div className='bg-[#f6f5f4] flex min-h-fit' id='TitleBar'>

            {title_bar_names.map((name) => (
                <div className='hover:bg-[#d8d7d7] px-2 py-1 round-md border-solid border-blue-50 font-sans text-xs' key={name}>{name}</div>
            ))}
        </div>
    )
}

// font-serif
// font-mono
// font-sans