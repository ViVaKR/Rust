# viv-camp

## Owernership (소유권)

> Owner : 값의 소유자는 값을 유지하는 변수 또는 데이터 구조이며 해당 데이터를 저장하는 데 사용되는 메모리를 할당하고 해제하는 역할을 합니다.

- Rust의 소유권 시스템은 고유하며 다른 프로그래밍 언어와 차별화됩니다.
- 메모리 관리를 관리하는 규칙 집합입니다.
- 규칙은 컴파일 시 적용됩니다.
- 규칙을 위반하면 프로그램이 컴파일되지 않습니다.
    - Each value in Rust has an owner (각 값에는 소유자가 있음)
    - There can only be one owner at a time (소유자는 한 번에 하나만 있을 수 있음)
    - When the owner goes out of scope, the value will be dropped. (소유자가 범위를 벗어나면 가치가 떨어집니다.)

## Basic Types

- Scalar
    - Integer :
        - i8, i16, i32, i64, i128, isize
        - u8, u16, u32, u64, u128, usize
            - decimal : `98_123`
            - hex : `0xff`
            - octal : `0x77`
            - binary : `0b111_0000`
            - byte (u8) : `b'a'`

    - Float
      - 
- Compound
    - Tuple Type
- Functions:
    - Block of reusable code that performs a specific tasks
    - Can take arguments, processes those inputs and return a result

- Diverging Functions:
    - Nerver return to the caller
    - E.g. panic, looping forever, quitting the program

## 스택 메모리

- 마지막이 먼저.
- 스택에 저장된 모든 데이터는 알려진 고정된 크기(예: 정수, 플로트, 차, 부 등)를 가져야 합니다.
- 새로운 데이터의 위치가 항상 스택의 맨 위에 있기 때문에 스택으로 푸시하는 것이 힙에 대한 할당보다 빠릅니다.
- 크기를 알 수 없는 유형이 힙에 할당되고 값에 대한 포인터가 스택에 푸시됩니다. 왜냐하면 포인터가 고정 크기이기 때문입니다.
- 스택 프레임
    - EBP 레지스터를 사용하여 스택 내의 지역변수 함수 인자값 리턴 주소에 접근한는 기법
    - 스택에는 지역변수 함수 인자값 리턴주소등을 구성됨
    - 스택 프레임은 함수가 호출될 때 생성되고 함수 처리가 완료되면 자동으로 소멸됨
    -

-----------
[ 낮은 주소 ]
-----------



------------
[ 높은 주소 ]
------------

```bash
$ cargo run -q --bin viv-camp

```






