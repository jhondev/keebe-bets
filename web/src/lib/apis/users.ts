import type { IUser } from "$lib/models/user"

export const users: Record<string,IUser> = {
    "79048457-f808-454b-b929-a6233ec695fb":{id: "79048457-f808-454b-b929-a6233ec695fb",name: "John",lastname: "Doe",email: "john.doe@example.com",phone: "123456789", user:"johndoe",password: "secretpassword",imageProfile: "/images/profile.png"},
    "f3bc0766-e1ad-4bfa-a374-5eeb4e9b9b1c":{id: "f3bc0766-e1ad-4bfa-a374-5eeb4e9b9b1c",name: "Charlie",lastname: "Adam",email: "john.doe@example.com",phone: "123456789", user:"johndoe",password: "secretpassword",imageProfile: "/images/profile.png"}
}


export const balances: Record<string,number> = {
    "79048457-f808-454b-b929-a6233ec695fb": 550,
    "f3bc0766-e1ad-4bfa-a374-5eeb4e9b9b1c": 720
}